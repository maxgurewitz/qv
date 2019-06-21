#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate actix_web;
extern crate diesel;

mod db;
mod middleware;
mod models;

use self::models::*;
use actix_cors::Cors;

use actix_web::error::ErrorInternalServerError;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use std::env;
use std::sync::Arc;

#[derive(Serialize)]
struct HomePageResource {
  user: Arc<Auth0Profile>,
}

static USER_NOT_FOUND_MSG: &str = "{ \"message\": \"Unable to locate user.\" }";

fn get_user_from_req(req: HttpRequest) -> Result<Arc<Auth0Profile>, Error> {
  let extensions = req.extensions();

  extensions
    .get::<middleware::AuthExtension>()
    .map(|e| e.user.clone())
    .ok_or_else(|| ErrorInternalServerError(USER_NOT_FOUND_MSG))
}

// routes
fn user_info_route(req: HttpRequest) -> Result<web::Json<HomePageResource>, Error> {
  let user = get_user_from_req(req)?;

  Ok(web::Json(HomePageResource { user: user }))
}

fn not_found_page_route(_req: HttpRequest) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::NotFound()
      .content_type("application/json")
      .body("{ \"message\": \"route not found\" }"),
  )
}

fn create_poll_route(
  _data: web::Data<middleware::AppData>,
  _req: HttpRequest,
) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn get_poll_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn update_proposal_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let poll_id = &req.match_info()["proposal_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn create_proposal_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn assign_vote_points_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let proposal_id = &req.match_info()["proposal_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn finish_voting(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn finish_poll(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

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

    let http_client = actix_web::client::Client::default();

    App::new()
      .wrap(Cors::new().send_wildcard())
      .data(middleware::AppData {
        pg_pool: pg_pool.clone(),
        _env: env,
        http_client,
      })
      .wrap(actix_web::middleware::Logger::default())
      .service(
        web::scope("/api")
          .service(
            web::scope("/private")
              .wrap(middleware::Auth)
              .route("/user-info", web::get().to(user_info_route))
              .service(
                web::scope("/poll")
                  .route("/", web::post().to(create_poll_route))
                  .service(
                    web::scope("/{poll_id}")
                      .route("/", web::get().to(get_poll_route))
                      .route("/finish-voting", web::put().to(finish_voting))
                      .route("/finish-poll", web::put().to(finish_poll))
                      .route("/proposal", web::post().to(create_proposal_route))
                  )
              )
              .service(
                web::scope("/proposal/{proposal_id}")
                  .route("/", web::put().to(update_proposal_route))
                  .route("/vote", web::put().to(assign_vote_points_route))
              )
          )
      )
      .default_service(web::route().to(not_found_page_route))
  })
  .bind(format!("0.0.0.0:{}", port))
  .expect("Can not bind to port")
  .run()
  .unwrap();
}
