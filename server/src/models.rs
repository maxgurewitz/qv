use super::schema::*;
use chrono::prelude::*;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Profile {
  pub email: String,
  pub email_verified: Option<bool>,
  pub name: Option<String>,
  pub locale: Option<String>,
  pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Poll {
  pub id: i32,
  pub email: String,
  pub title: String,
  pub poll_type: String,
  pub current_progress: ProgressEnum,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePollPayload {
  pub title: String,
  pub poll_type: String,
}

#[derive(Insertable)]
#[table_name="poll"]
pub struct NewPoll<'a> {
  pub email: &'a str,
  pub title: &'a str,
  pub poll_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteUserPayload {
  pub email: String
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResource {
  pub user: Arc<Auth0Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePollResource {
  pub poll: Poll,
}