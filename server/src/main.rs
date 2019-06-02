#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix_web;

mod db;
mod middleware;
mod models;
mod schema;

use self::models::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::prelude::*;
use rand::Rng;
use std::env;

static _JWT_PUBLIC_KEY: &str = "{\"keys\":[{\"alg\":\"RS256\",\"kty\":\"RSA\",\"use\":\"sig\",\"x5c\":[\"MIIDBTCCAe2gAwIBAgIJXNFmm/00aDEeMA0GCSqGSIb3DQEBCwUAMCAxHjAcBgNVBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTAeFw0xODA3MDEwNjU1MTFaFw0zMjAzMDkwNjU1MTFaMCAxHjAcBgNVBAMTFW1heHRoZWdlZWsxLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBANKfuFitIpFrJbgm8JENTlwOLDZWvMidE2zCSHlpyotQdDohFKfOHqs/Hjj9DJ8AzIw0q3N+Xc3gt8klPOm6Ix/D55Q4DECQO/orGhyCL0NkuYKn6iGAS4hRwgrz9syCVfDQEe/K1PUC9AnfBGgj9SDxScO7sjRaMjTqxscphrB7sAXtgKvVRERuaQxc8JeX2x/HGMUNrJlFho2s/sn+UP6fH5Ix1vfIB1w3ixRiku9Qp1nCAkVTBCPIVRBm+9Hq1UohE+uBCkXQ6+fxEF2h+7p4VEgoR3eV4psBsZX46jOeEucucxPzPNhoNx7S67MViPJuIlkNG8uZB1ag6flX+g0CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUHXj0hn1+jKHAim02ffhpegWRL5AwDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQBEs/pBb+YbjLwdwFMmVIgA8mzduXJxleAtWl1ffKxjG57ApJ8xLuc2vIoygB5rX/kNZZgTyZzTvdPg2rbWCNsONUzxic4eDAcuPHGalN9VlB03QH29uEWyxYa0sL1FlToQbYglT5pkS68F6wbOxHSqZFuFvKmtaRPHNJZqMJLVx9GuOchozjllrGiZ6ko5iu7ePRkM44IXgp5+Bq4cDOWV41lFEOg5ClLXGh/PIhHxOKnKGuWxfHBHu8p8LwQ5w9cqDye88rEBqO/WMNb6TYCu6HRxVPKwVRsF8ZeBN2Bc1EpRnWw3ffMbxGNwag0otCNnWf8eCGGiEG3UXDLBMN2T\"],\"n\":\"0p-4WK0ikWsluCbwkQ1OXA4sNla8yJ0TbMJIeWnKi1B0OiEUp84eqz8eOP0MnwDMjDSrc35dzeC3ySU86bojH8PnlDgMQJA7-isaHIIvQ2S5gqfqIYBLiFHCCvP2zIJV8NAR78rU9QL0Cd8EaCP1IPFJw7uyNFoyNOrGxymGsHuwBe2Aq9VERG5pDFzwl5fbH8cYxQ2smUWGjaz-yf5Q_p8fkjHW98gHXDeLFGKS71CnWcICRVMEI8hVEGb70erVSiET64EKRdDr5_EQXaH7unhUSChHd5XimwGxlfjqM54S5y5zE_M82Gg3HtLrsxWI8m4iWQ0by5kHVqDp-Vf6DQ\",\"e\":\"AQAB\",\"kid\":\"OEQ0MTE1NkYyNTVFQkNFQkFGQ0UyMDZDN0EzQjg1NDcyNEQ3QjJBMQ\",\"x5t\":\"OEQ0MTE1NkYyNTVFQkNFQkFGQ0UyMDZDN0EzQjg1NDcyNEQ3QjJBMQ\"}]}";

#[derive(Serialize)]
struct HomePageResource {
    name: String,
}

// routes
fn home_page_route(_req: HttpRequest) -> Result<web::Json<HomePageResource>, Error> {
    Ok(web::Json(HomePageResource {
        name: "foo".to_string(),
    }))
}

fn not_found_page_route(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound()
        .content_type("application/json")
        .body("{ \"message\": \"route not found\" }"))
}

fn login_route(
    data: web::Data<middleware::AppData>,
    _reg: HttpRequest,
) -> Result<HttpResponse, Error> {
    use schema::users;

    // TODO error handling
    let connection = data.pg_pool.get().unwrap();

    let mut rng = rand::thread_rng();
    let num: f32 = rng.gen();

    let new_user = NewUser {
        openid: format!("test-user-{}", num.to_string()),
        email: "test@email.com",
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&*connection)
        .expect("Error saving new post");

    // create user if not found
    Ok(HttpResponse::NotFound()
        .content_type("application/json")
        .body("{ \"message\": \"success\" }"))
}

// auth flow https://auth0.com/docs/flows/concepts/auth-code
// https://auth0.com/docs/flows/guides/implicit/add-login-implicit
// https://auth0.com/docs/architecture-scenarios/spa-api
fn main() {
    let port = env::var("PORT").unwrap_or("8000".to_string());

    println!("Attempting to bind to port: {}", port);

    HttpServer::new(|| {
        let pg_pool = db::build_pg_pool();

        let env: middleware::Env = env::var("ENV")
            .map(|s| {
                if s == "dev" {
                    middleware::Env::Dev
                } else {
                    middleware::Env::Prod
                }
            })
            .unwrap_or(middleware::Env::Prod);

        App::new()
            .data(middleware::AppData {
                pg_pool: pg_pool.clone(),
                _env: env,
            })
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/private/")
                            .wrap(middleware::Auth)
                            .route("/home", web::post().to(home_page_route)),
                    )
                    .route("/login", web::post().to(login_route)),
            )
            .default_service(web::route().to(not_found_page_route))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Can not bind to port")
    .run()
    .unwrap();
}
