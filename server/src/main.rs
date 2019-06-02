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
use actix_web::error::ErrorInternalServerError;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use diesel::prelude::*;
use rand::Rng;
use std::env;
use std::sync::Arc;

#[derive(Serialize)]
struct HomePageResource {
    name: String,
}

static USER_NOT_FOUND_MSG: &str = "{ \"message\": \"Unable to locate user.\" }";

fn get_user_from_req(req: HttpRequest) -> Result<Arc<NewUser>, Error> {
    let extensions = req.extensions();

    extensions
        .get::<middleware::AuthExtension>()
        .map(|e| e.user.clone())
        .ok_or_else(|| ErrorInternalServerError(USER_NOT_FOUND_MSG))
}

// routes
fn home_page_route(req: HttpRequest) -> Result<web::Json<HomePageResource>, Error> {
    let _user = get_user_from_req(req)?;

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
        email: "test@email.com".to_string(),
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
