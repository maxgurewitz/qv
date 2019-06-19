use super::db;
use super::models::Auth0Profile;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use actix_utils::cloneable::CloneableService;
use actix_web::web::BytesMut;
use futures::{Future, Stream};

use std::collections::HashMap;

use futures::future::{FutureResult, ok};
use futures::Poll;

use std::sync::Arc;

static USER_INFO_URL: &str = "https://maxthegeek1.auth0.com/userinfo";

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    args: HashMap<String, String>,
    data: String,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: Auth0Profile,
    origin: String,
    url: String,
}

// Implemented from example: 
// https://github.com/actix/examples/blob/aa5750cafc9d6bc96e098c338b70fa618149ffd5/async_ex1/src/main.rs
fn fetch_user_info(
    client: &actix_web::client::Client,
    bearer_token: &String,
) -> impl Future<Item = Auth0Profile, Error = Error> {
    client
        .get(&USER_INFO_URL.to_string())
        .header(
          "Authorization",
          format!("Bearer {}", bearer_token),
        )
        .send()
        .map_err(|e| {
          println!("Unable to parse bearer token: {}", e);
          ErrorUnauthorized(INVALID_TOKEN_MSG)
        })
        .and_then(|resp| {
            resp.from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                  acc.extend_from_slice(&chunk);
                  Ok::<_, Error>(acc)
                })
                .and_then(|body| {
                  serde_json::from_slice(&body).map_err(|e| {
                    println!("Response has unexpected structure: {}", e);
                    ErrorUnauthorized(INVALID_TOKEN_MSG)
                  })
                })
        })
}

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthMiddleware<S>;
  type Future = FutureResult<Self::Transform, Self::InitError>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AuthMiddleware { service: CloneableService::new(service) })
  }
}

pub struct AuthMiddleware<S> {
  service: CloneableService<S>,
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

fn get_user_info_args(req: &ServiceRequest) -> Result<(String, actix_web::client::Client), actix_web::Error> {
  let bearer = parse_bearer(&req).ok_or(ErrorUnauthorized(MISSING_HEADER_MSG))?;

  let app_data = req
    .app_data::<AppData>()
    .ok_or(ErrorInternalServerError(INTERNAL_SERVICE_ERROR_MSG))?;

  Result::Ok((bearer, app_data.http_client.clone()))
}

fn fetch_user_info_from_req(req: &ServiceRequest) -> impl Future<Item = Auth0Profile, Error = Error> {
  let user_info = get_user_info_args(&req);
  FutureResult::from(user_info)
  .and_then(|(bearer, client)|
    fetch_user_info(&client, &bearer)
  )
}

impl<S, B> Service for AuthMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

  fn poll_ready(&mut self) -> Poll<(), Self::Error> {
    self.service.poll_ready()
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
      let user_info_fut = fetch_user_info_from_req(&req);

      // modeled after example in actix-redix 
      let mut srv = self.service.clone();

      Box::new(user_info_fut
        .and_then(move |user_info| {
          let extension = AuthExtension {
            user: Arc::new(user_info),
          };

          req.extensions_mut().insert(extension);
          srv.call(req)
        }))
  }
}

pub struct AppData {
  pub pg_pool: db::PgPool,
  pub _env: Env,
  pub http_client: actix_web::client::Client
}

#[derive(Clone, Copy)]
pub enum Env {
  Dev,
  Prod,
}