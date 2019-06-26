#[macro_use]
extern crate serde_derive;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate chrono;

mod db;
mod middleware;
mod models;
mod schema;
mod static_responses;
use models::*;
use static_responses::*;
use actix_web::error::{ErrorInternalServerError, ErrorBadRequest};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::{Json};
use std::env;
use std::sync::Arc;
use diesel::prelude::*;

fn get_user_from_req(req: HttpRequest) -> Result<Arc<Auth0Profile>, Error> {
  let extensions = req.extensions();

  extensions
    .get::<middleware::AuthExtension>()
    .map(|e| e.user.clone())
    .ok_or_else(|| ErrorInternalServerError(NOT_FOUND_MSG))
}

fn map_to_intern_service_err<T>(_: T) -> Error {
  ErrorInternalServerError(INTERNAL_SERVICE_ERROR_MSG)
}

// routes
fn user_info_route(req: HttpRequest) -> Result<web::Json<UserInfoResource>, Error> {
  let user = get_user_from_req(req)?;

  Ok(web::Json(UserInfoResource { user: user }))
}

fn not_found_page_route(_req: HttpRequest) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::NotFound()
      .content_type("application/json")
      .body(NOT_FOUND_MSG),
  )
}

fn create_poll_route(
  data: web::Data<middleware::AppData>,
  payload: Json<CreatePollPayload>,
  req: HttpRequest,
) -> Result<Json<CreatePollResource>, Error> {
  use schema::poll;

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_intern_service_err)?;
  
  let user_info = get_user_from_req(req)
    .map_err(map_to_intern_service_err)?;

  let new_poll = NewPoll {
    email: &user_info.email,
    title: &payload.title,
    poll_type: &payload.poll_type,
  };

  let poll = diesel::insert_into(poll::table)
    .values(&new_poll)
    .get_result::<Poll>(&*connection)
    .map_err(map_to_intern_service_err)?;

  Ok(web::Json(CreatePollResource { poll }))
}

fn get_poll_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let _poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(GENERIC_SUCCESS_MSG),
  )
}

fn update_proposal_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let _poll_id = &req.match_info()["proposal_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn create_proposal_route(
  data: web::Data<middleware::AppData>,
  payload: Json<CreateProposalPayload>,
  req: HttpRequest,
) -> Result<Json<CreateProposalResource>, Error> {
  use schema::proposal;

  let poll_id = &req.match_info()["poll_id"].parse::<i32>()
    .map_err(|_| ErrorBadRequest("{ \"message\": \"poll_id path param must be an integer\" }"))?;

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_intern_service_err)?;

  let new_proposal = NewProposal {
    summary: &payload.summary,
    full_description_link: payload.full_description_link.clone(),
    poll_id: poll_id,
  };

  let proposal = diesel::insert_into(proposal::table)
    .values(&new_proposal)
    .get_result::<Proposal>(&*connection)
    .map_err(map_to_intern_service_err)?;

  Ok(Json(CreateProposalResource { proposal }))
}

fn assign_vote_points_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let _proposal_id = &req.match_info()["proposal_id"];

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
  let _poll_id = &req.match_info()["poll_id"];

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
  let _poll_id = &req.match_info()["poll_id"];

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn user_search(
  _data: web::Data<middleware::AppData>,
  _req: HttpRequest,
) -> Result<HttpResponse, Error> {

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn invite_user(
  _data: web::Data<middleware::AppData>,
  _invite_user_payload: Json<InviteUserPayload>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let _poll_id = &req.match_info()["poll_id"];

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
              .route("/user-search", web::get().to(user_search))
              .service(
                web::scope("/poll")
                  .route("", web::post().to(create_poll_route))
                  .service(
                    web::scope("/{poll_id}")
                      .route("", web::get().to(get_poll_route))
                      .route("/invite-user", web::post().to(invite_user))
                      .route("/finish-voting", web::put().to(finish_voting))
                      .route("/finish-poll", web::put().to(finish_poll))
                      .route("/proposal", web::post().to(create_proposal_route))
                  )
              )
              .service(
                web::scope("/proposal/{proposal_id}")
                  .route("", web::put().to(update_proposal_route))
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
