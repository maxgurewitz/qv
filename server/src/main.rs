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
mod sql_enum_types;
mod static_responses;
use models::*;
use diesel::dsl::{sum};
use static_responses::*;
use actix_web::error::{ErrorInternalServerError};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::{Json};
use std::env;
use std::sync::Arc;
use std::fmt::{Display};
use diesel::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_user_from_req(req: &HttpRequest) -> Result<Arc<Auth0Profile>, Error> {
  let extensions = req.extensions();

  extensions
    .get::<middleware::AuthExtension>()
    .map(|e| e.user.clone())
    .ok_or_else(|| ErrorInternalServerError(NOT_FOUND_MSG))
}

fn map_to_internal_service_err<T: Display>(e: T) -> HttpResponse {
  println!("Programmatic error: {}", e);

  HttpResponse::InternalServerError()
    .json(GenericJsonResponse { message: INTERNAL_SERVICE_ERROR_MSG.to_string() })
}

// routes
fn user_info_route(req: HttpRequest) -> Result<web::Json<UserInfoResource>, Error> {
  let user = get_user_from_req(&req)?;

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
  use schema::polls;

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;
  
  let user_info = get_user_from_req(&req)?;

  let new_poll = NewPoll {
    email: &user_info.email,
    title: &payload.title,
    poll_type: &payload.poll_type,
  };

  let poll = diesel::insert_into(polls::table)
    .values(&new_poll)
    .get_result::<Poll>(&*connection)
    .map_err(map_to_internal_service_err)?;

  Ok(web::Json(CreatePollResource { poll }))
}

fn get_poll_route(
  data: web::Data<middleware::AppData>,
  poll_id_param: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  use schema::{polls, votes, proposals};

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  // FIXME check that user owns or was invited to poll
  let poll_id = poll_id_param.into_inner();

  let poll = polls::table
      .find(&poll_id)
      .first::<Poll>(&*connection)
      .optional()
      .map_err(map_to_internal_service_err)?
      .ok_or_else(|| 
        HttpResponse::NotFound()
          .json(GenericJsonResponse { 
            message: "Poll not found.".to_string() 
          })
      )?;

  let point_totals = if poll.current_progress == sql_enum_types::ProgressEnum::Finished {
    let vote_points = votes::table
        .inner_join(proposals::table.inner_join(polls::table))
        .filter(polls::dsl::id.eq(&poll_id))
        .select((votes::dsl::points, votes::dsl::proposal_id))
        .load::<(f64, i32)>(&*connection)
        .map_err(map_to_internal_service_err)?;

    let mut totals: HashMap<i32, f64> = HashMap::new();
    for vote in &vote_points {
      // Take the square root of point totals as outlined by QV.
      let points = if vote.0.is_sign_negative() {
        - vote.0.abs().sqrt()
      } else {
        vote.0.sqrt()
      };
      let proposal_id = vote.1;

      let sum_for_proposal = totals
        .entry(proposal_id)
        .or_insert(0.0);

      *sum_for_proposal += points;
    }
    Option::Some(totals)
  } else {
    Option::None
  };

  let assigned_proposals = if poll.current_progress != sql_enum_types::ProgressEnum::NotStarted {
    proposals::table
      .filter(proposals::dsl::poll_id.eq(&poll_id))
      .load::<Proposal>(&*connection)
      .map(Option::Some)
      .map_err(map_to_internal_service_err)?
  } else {
    Option::None
  };

  let resource = GetPollResource {
    point_totals,
    proposals: assigned_proposals,
    poll
  };

  Ok(HttpResponse::Ok().json(resource))
}

fn update_proposal_route(
  _data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  let _poll_id = &req.match_info()["proposal_id"];
  // FIXME check that user is owner

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn create_proposal_route(
  data: web::Data<middleware::AppData>,
  payload: Json<CreateProposalPayload>,
  poll_id: actix_web::web::Path<i32>,
) -> Result<Json<CreateProposalResource>, Error> {
  use schema::proposals;
  // FIXME check that user owns poll
  // FIXME check that poll is unstarted

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  let new_proposal = NewProposal {
    summary: &payload.summary,
    full_description_link: payload.full_description_link.clone(),
    poll_id: &poll_id,
  };

  let proposal = diesel::insert_into(proposals::table)
    .values(&new_proposal)
    .get_result::<Proposal>(&*connection)
    .map_err(map_to_internal_service_err)?;

  Ok(Json(CreateProposalResource { proposal }))
}

fn assign_vote_points_route(
  data: web::Data<middleware::AppData>,
  req: HttpRequest,
  payload: Json<CreateVotePayload>,
  proposal_id_param: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  use schema::{user_invites, proposals, user_invite_locks, polls, votes};
  let proposal_id = proposal_id_param.into_inner();

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  let user_info = get_user_from_req(&req)?;

  connection.transaction::<_, diesel::result::Error, _>(|| {
    let (user_invite_id, poll_progress) = user_invites::table
      .inner_join(polls::table.inner_join(proposals::table))
      .filter(proposals::dsl::id.eq(&proposal_id)
        .and(user_invites::dsl::email.eq(&user_info.email)))
      .select((user_invites::id, polls::current_progress))
      .first::<(i32, sql_enum_types::ProgressEnum)>(&*connection)?;

    // lock in order to prevent race conditions resulting in the user being able to spend more than their allotted points
    user_invite_locks::table
      .find(&user_invite_id)
      .for_update()
      .execute(&*connection)?;

    let vote_sum = votes::table
      .filter(votes::dsl::user_invite_id.eq(&user_invite_id))
      .select(sum(votes::dsl::points))
      .first::<Option<f64>>(&*connection)?
      .unwrap_or(0.0);

    let vote_to_overwrite_option = votes::table
      .filter(votes::dsl::user_invite_id.eq(&user_invite_id)
        .and(votes::dsl::proposal_id.eq(&proposal_id)))
      .first::<Vote>(&*connection)
      .optional()?;

    let within_budget = (vote_sum - vote_to_overwrite_option.map(|v| v.points).unwrap_or(0.0) + payload.points) < 100.0;

    if poll_progress == sql_enum_types::ProgressEnum::InProgress &&within_budget {
      vote_to_overwrite_option.map(|vote_to_overwrite| 
        diesel::update(votes::dsl::votes.find(&vote_to_overwrite.id))
          .set(votes::dsl::points.eq(&payload.points))
          .execute(&*connection)
      ).unwrap_or_else(|| {
        let new_vote = NewVote {
          proposal_id: &proposal_id,
          user_invite_id: &user_invite_id,
          points: &payload.points,
        };

        diesel::insert_into(votes::table)
          .values(&new_vote)
          .execute(&*connection)
      })?;
    }
    // TODO handle failure due to lack of points or not in progress and respond with 400

    Ok(())
  })
  .map_err(|e| {
    match e {
      diesel::result::Error::NotFound => 
        HttpResponse::Forbidden()
          .json(GenericJsonResponse { 
            message: "User lacks access to the relevant poll.".to_string() 
          }),
      _ => map_to_internal_service_err(e)
    }
  })?;

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn get_my_votes(
  _data: web::Data<middleware::AppData>,
  _poll_id: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn start_poll(
  data: web::Data<middleware::AppData>,
  poll_id: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  use schema::polls;
  use schema::polls::dsl::*;
  // FIXME check that user owns poll

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  let inner_poll_id = poll_id.into_inner();
  let target = id.eq(inner_poll_id).and(current_progress.eq(sql_enum_types::ProgressEnum::NotStarted));

  diesel::update(polls)
    .filter(target)
    .set(polls::current_progress.eq(sql_enum_types::ProgressEnum::InProgress))
    .execute(&*connection)
    // TODO handle 404's
    .map_err(map_to_internal_service_err)?; 

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body("{ \"message\": \"success\" }"),
  )
}

fn finish_poll(
  data: web::Data<middleware::AppData>,
  poll_id_param: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  use schema::polls;
  use schema::polls::dsl::*;
  // FIXME check that user owns poll

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  let poll_id = poll_id_param.into_inner();

  let target = id.eq(poll_id).and(current_progress.eq(sql_enum_types::ProgressEnum::InProgress));

  diesel::update(polls)
    .filter(target)
    .set(polls::current_progress.eq(sql_enum_types::ProgressEnum::Finished))
    .execute(&*connection)
    // TODO handle 404's
    .map_err(map_to_internal_service_err)?; 

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

fn home_route(
  data: web::Data<middleware::AppData>,
  req: HttpRequest,
) -> Result<HttpResponse, Error> {
  use schema::{polls, user_invites};

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  let user = get_user_from_req(&req)?;

  let admin_polls = polls::table
    .filter(polls::dsl::email.eq(&user.email))
    .load::<Poll>(&*connection)
    .map_err(map_to_internal_service_err)?;
  
  let invite_polls = polls::table
    .inner_join(user_invites::table)
    .filter(user_invites::dsl::email.eq(&user.email))
    .select(polls::all_columns)
    .load::<Poll>(&*connection)
    .map_err(map_to_internal_service_err)?;

  let mut poll_set: HashSet<Poll> = HashSet::new();
  let mut admin_poll_ids: Vec<i32> = Vec::new();
  let mut invite_poll_ids: Vec<i32> = Vec::new();

  for poll in admin_polls {
    admin_poll_ids.push(poll.id);
    poll_set.insert(poll);
  }
  for poll in invite_polls {
    invite_poll_ids.push(poll.id);
    poll_set.insert(poll);
  }

  let resource = HomeResource {
    polls: poll_set,
    invite_poll_ids,
    admin_poll_ids
  };

  Ok(
    HttpResponse::Ok()
      .json(resource)
  )
}

fn invite_user(
  data: web::Data<middleware::AppData>,
  payload: Json<InviteUserPayload>,
  poll_id: actix_web::web::Path<i32>,
) -> Result<HttpResponse, Error> {
  use schema::{user_invites, user_invite_locks};

  let connection = data
    .pg_pool
    .get()
    .map_err(map_to_internal_service_err)?;

  // FIXME check that user owns poll
  connection.transaction::<_, diesel::result::Error, _>(|| {
    let new_user_invite = NewUserInvite {
      email: &payload.email,
      poll_id: &poll_id
    };

    let user_invite = diesel::insert_into(user_invites::table)
      .values(&new_user_invite)
      .get_result::<UserInvite>(&*connection)?;

    let new_user_invite_lock = NewUserInviteLock {
      user_invite_id: &user_invite.id
    };

    diesel::insert_into(user_invite_locks::table)
      .values(&new_user_invite_lock)
      .execute(&*connection)?;

    Ok(())
  })
  .map_err(map_to_internal_service_err)?;

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .json(GenericJsonResponse { message: "Succesfully invited user.".to_string() }),
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
              .route("/home", web::get().to(home_route))
              .service(
                web::scope("/polls")
                  .route("", web::post().to(create_poll_route))
                  .service(
                    web::scope("/{poll_id}")
                      .route("", web::get().to(get_poll_route))
                      .route("/my-votes", web::get().to(get_my_votes))
                      .route("/invite-user", web::post().to(invite_user))
                      .route("/start", web::put().to(start_poll))
                      .route("/finish", web::put().to(finish_poll))
                      .route("/proposals", web::post().to(create_proposal_route))
                  )
              )
              .service(
                web::scope("/proposals/{proposal_id}")
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
