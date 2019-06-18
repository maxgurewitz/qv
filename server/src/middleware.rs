extern crate reqwest;
use super::db;
use super::models::Auth0Profile;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorUnauthorized, ErrorInternalServerError};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use futures::future::{err, ok, Either, FutureResult};
use futures::Poll;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    email_verified: bool,
}

static USER_INFO_URL: &str = "https://maxthegeek1.auth0.com/userinfo";

fn request_user_info(bearer_token: &str, app_data: &AppData) -> Result<Auth0Profile, reqwest::Error> {
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
    pub user: Arc<Auth0Profile>,
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
static INTERNAL_SERVICE_ERROR_MSG: &str = "{ \"message\": \"Oops something went wrong.\" }";

fn get_user_info(req: &ServiceRequest) -> Result<Auth0Profile, actix_web::Error> {
    let bearer = parse_bearer(&req).ok_or(ErrorUnauthorized(MISSING_HEADER_MSG))?;

    let app_data = req
        .app_data::<AppData>()
        .ok_or_else(|| ErrorInternalServerError(INTERNAL_SERVICE_ERROR_MSG))?;

    request_user_info(&bearer, &app_data).map_err(|_| ErrorUnauthorized(INVALID_TOKEN_MSG))
}

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
        get_user_info(&req)
          .map(|user_info| {
            let extension = AuthExtension {
                user: Arc::new(user_info),
            };

            req.extensions_mut().insert(extension);
            Either::A(self.service.call(req))
          })
          .unwrap_or_else(|http_err| Either::B(err(http_err)))
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