extern crate jsonwebtoken as jwt;
extern crate reqwest;
use super::db;
use super::models::NewUser;
use super::queries;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use futures::future::{err, ok, Either, FutureResult};
use futures::Poll;
use jwt::{decode, encode, Algorithm, Header, Validation};
use serde_json::value::Value;
use std::sync::Arc;

// https://github.com/actix/examples/blob/master/middleware/src/simple.rs
// https://github.com/actix/examples/blob/9c8389e06e75ac704af0ccb91865c321684d8bf0/simple-auth-server/src/utils.rs
// https://crates.io/crates/actix-web-httpauth
// https://dev.to/mygnu/auth-web-microservice-with-rust-using-actix-web---complete-tutorial-part-2-k3a
// https://www.jamesbaum.co.uk/blether/creating-authentication-middleware-actix-rust-react/
// https://github.com/actix/actix-web/issues/300
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    email_verified: bool,
}

static USER_INFO_URL: &str = "https://maxthegeek1.auth0.com/userinfo";

fn request_user_info(bearer_token: &str, app_data: &AppData) -> Result<NewUser, reqwest::Error> {
    let mut response = app_data
        .http_client
        .get(&USER_INFO_URL.to_string())
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", bearer_token),
        )
        .send()?;
    response.json()
}

// FIXME using public key, should be using private?
static _JWT_PUBLIC_KEY: &str = "-----BEGIN CERTIFICATE-----\nMIIDBTCCAe2gAwIBAgIJXNFmm/00aDEeMA0GCSqGSIb3DQEBCwUAMCAxHjAcBgNV\nBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTAeFw0xODA3MDEwNjU1MTFaFw0zMjAz\nMDkwNjU1MTFaMCAxHjAcBgNVBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTCCASIw\nDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBANKfuFitIpFrJbgm8JENTlwOLDZW\nvMidE2zCSHlpyotQdDohFKfOHqs/Hjj9DJ8AzIw0q3N+Xc3gt8klPOm6Ix/D55Q4\nDECQO/orGhyCL0NkuYKn6iGAS4hRwgrz9syCVfDQEe/K1PUC9AnfBGgj9SDxScO7\nsjRaMjTqxscphrB7sAXtgKvVRERuaQxc8JeX2x/HGMUNrJlFho2s/sn+UP6fH5Ix\n1vfIB1w3ixRiku9Qp1nCAkVTBCPIVRBm+9Hq1UohE+uBCkXQ6+fxEF2h+7p4VEgo\nR3eV4psBsZX46jOeEucucxPzPNhoNx7S67MViPJuIlkNG8uZB1ag6flX+g0CAwEA\nAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUHXj0hn1+jKHAim02ffhp\negWRL5AwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQBEs/pBb+Yb\njLwdwFMmVIgA8mzduXJxleAtWl1ffKxjG57ApJ8xLuc2vIoygB5rX/kNZZgTyZzT\nvdPg2rbWCNsONUzxic4eDAcuPHGalN9VlB03QH29uEWyxYa0sL1FlToQbYglT5pk\nS68F6wbOxHSqZFuFvKmtaRPHNJZqMJLVx9GuOchozjllrGiZ6ko5iu7ePRkM44IX\ngp5+Bq4cDOWV41lFEOg5ClLXGh/PIhHxOKnKGuWxfHBHu8p8LwQ5w9cqDye88rEB\nqO/WMNb6TYCu6HRxVPKwVRsF8ZeBN2Bc1EpRnWw3ffMbxGNwag0otCNnWf8eCGGi\nEG3UXDLBMN2T\n-----END CERTIFICATE-----\n";
// static _JWT_PUBLIC_KEY: &str = "{\"keys\":[{\"alg\":\"RS256\",\"kty\":\"RSA\",\"use\":\"sig\",\"x5c\":[\"MIIDBTCCAe2gAwIBAgIJXNFmm/00aDEeMA0GCSqGSIb3DQEBCwUAMCAxHjAcBgNVBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTAeFw0xODA3MDEwNjU1MTFaFw0zMjAzMDkwNjU1MTFaMCAxHjAcBgNVBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBANKfuFitIpFrJbgm8JENTlwOLDZWvMidE2zCSHlpyotQdDohFKfOHqs/Hjj9DJ8AzIw0q3N+Xc3gt8klPOm6Ix/D55Q4DECQO/orGhyCL0NkuYKn6iGAS4hRwgrz9syCVfDQEe/K1PUC9AnfBGgj9SDxScO7sjRaMjTqxscphrB7sAXtgKvVRERuaQxc8JeX2x/HGMUNrJlFho2s/sn+UP6fH5Ix1vfIB1w3ixRiku9Qp1nCAkVTBCPIVRBm+9Hq1UohE+uBCkXQ6+fxEF2h+7p4VEgoR3eV4psBsZX46jOeEucucxPzPNhoNx7S67MViPJuIlkNG8uZB1ag6flX+g0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUHXj0hn1+jKHAim02ffhpegWRL5AwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQBEs/pBb+YbjLwdwFMmVIgA8mzduXJxleAtWl1ffKxjG57ApJ8xLuc2vIoygB5rX/kNZZgTyZzTvdPg2rbWCNsONUzxic4eDAcuPHGalN9VlB03QH29uEWyxYa0sL1FlToQbYglT5pkS68F6wbOxHSqZFuFvKmtaRPHNJZqMJLVx9GuOchozjllrGiZ6ko5iu7ePRkM44IXgp5+Bq4cDOWV41lFEOg5ClLXGh/PIhHxOKnKGuWxfHBHu8p8LwQ5w9cqDye88rEBqO/WMNb6TYCu6HRxVPKwVRsF8ZeBN2Bc1EpRnWw3ffMbxGNwag0otCNnWf8eCGGiEG3UXDLBMN2T\"],\"n\":\"0p-4WK0ikWsluCbwkQ1OXA4sNla8yJ0TbMJIeWnKi1B0OiEUp84eqz8eOP0MnwDMjDSrc35dzeC3ySU86bojH8PnlDgMQJA7-isaHIIvQ2S5gqfqIYBLiFHCCvP2zIJV8NAR78rU9QL0Cd8EaCP1IPFJw7uyNFoyNOrGxymGsHuwBe2Aq9VERG5pDFzwl5fbH8cYxQ2smUWGjaz-yf5Q_p8fkjHW98gHXDeLFGKS71CnWcICRVMEI8hVEGb70erVSiET64EKRdDr5_EQXaH7unhUSChHd5XimwGxlfjqM54S5y5zE_M82Gg3HtLrsxWI8m4iWQ0by5kHVqDp-Vf6DQ\",\"e\":\"AQAB\",\"kid\":\"OEQ0MTE1NkYyNTVFQkNFQkFGQ0UyMDZDN0EzQjg1NDcyNEQ3QjJBMQ\",\"x5t\":\"OEQ0MTE1NkYyNTVFQkNFQkFGQ0UyMDZDN0EzQjg1NDcyNEQ3QjJBMQ\"}]}";
static _DOMAIN: &str = "https://shrouded-waters-56080.herokuapp.com/";

// FIXME rather than decode manually hit auth0 endpoint
// https://auth0.com/docs/api/authentication#get-user-info
fn _decode_jwt(token: &String) -> Option<Claims> {
    decode::<Claims>(
        &token,
        _JWT_PUBLIC_KEY.as_ref(),
        &Validation {
            algorithms: vec![Algorithm::RS256],
            // aud: Option::Some(Value::String(_DOMAIN.to_string())),
            // iss: Option::Some(_DOMAIN.to_string()),
            ..Validation::default()
        },
    )
    .map_err(|e| {
        println!("Decoding failure {}", e);
        e
    })
    .ok()
    .map(|token| token.claims)
}

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

pub struct AuthExtension {
    pub user: Arc<NewUser>,
}

fn parse_bearer(req: &ServiceRequest) -> Option<String> {
    let header = req.headers().get(AUTHORIZATION)?;
    // "Bearer *" length
    if header.len() < 8 {
        return Option::None;
    }

    let mut parts = header.to_str().ok()?.splitn(2, ' ');

    match parts.next() {
        Some(scheme) if scheme == "Bearer" => (),
        _ => return Option::None,
    }

    let token = parts.next()?;

    Option::from(token.to_string())
}

static MISSING_HEADER_MSG: &str = "{ \"message\": \"Must pass Authorization Bearer header.\" }";
static INVALID_TOKEN_MSG: &str = "{ \"message\": \"Invalid Bearer header.\" }";

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let maybe_bearer = parse_bearer(&req);

        // FIXME there should be a better pattern
        if maybe_bearer.is_none() {
            return Either::B(err(ErrorUnauthorized(MISSING_HEADER_MSG)));
        }

        let bearer = maybe_bearer.unwrap();
        println!("Bearer: {}", bearer);

        let app_data = req
            .app_data::<AppData>()
            .expect("Programmatic error, app data not initialized.");

        let user_info_result = request_user_info(&bearer, &app_data);

        if user_info_result.is_err() {
            return Either::B(err(ErrorUnauthorized(INVALID_TOKEN_MSG)));
        }
        let user_info = user_info_result.unwrap();

        let _query_result = app_data.pg_pool.get();

        let extension = AuthExtension {
            user: Arc::new(user_info),
        };

        req.extensions_mut().insert(extension);
        Either::A(self.service.call(req))
    }
}

pub struct AppData {
    pub pg_pool: db::PgPool,
    pub _env: Env,
    pub http_client: reqwest::Client,
}

#[derive(Clone, Copy)]
pub enum Env {
    Dev,
    Prod,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_decode_jwt() {
        let sample_token = "z2M9CVSg0lkwFTS1FEk6gjf-jncdcu-M";
        let claims = _decode_jwt(&sample_token.to_string()).unwrap();
        assert_eq!(claims.sub, "foo");
    }
}
