use super::schema::*;
use chrono::prelude::*;
use std::sync::Arc;
use super::sql_enum_types::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Profile {
  pub email: String,
  pub email_verified: Option<bool>,
  pub name: Option<String>,
  pub locale: Option<String>,
  pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Eq, PartialEq, Hash, Clone)]
pub struct Poll {
  pub id: i32,
  pub email: String,
  pub title: String,
  pub summary: String,
  pub full_description_link: Option<String>,
  pub poll_type: String,
  pub current_progress: ProgressEnum,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Proposal {
  pub id: i32,
  pub summary: String,
  pub full_description_link: Option<String>,
  pub poll_id: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct UserInvite {
  pub id: i32,
  pub email: String,
  pub poll_id: i32,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Copy, Clone)]
pub struct Vote {
  pub id: i32,
  pub user_invite_id: i32,
  pub proposal_id: i32,
  pub points: f64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePollPayload {
  pub title: String,
  pub poll_type: String,
  pub summary: String,
  pub full_description_link: Option<String>,
}

#[derive(Insertable)]
#[table_name="polls"]
pub struct NewPoll<'a> {
  pub email: &'a str,
  pub title: &'a str,
  pub summary: &'a str,
  pub full_description_link: Option<String>,
  pub poll_type: &'a str,
}

#[derive(Insertable)]
#[table_name="proposals"]
pub struct NewProposal<'a> {
  // TODO add title
  pub summary: &'a str,
  pub full_description_link: Option<String>,
  pub poll_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="user_invites"]
pub struct NewUserInvite<'a> {
  pub email: &'a str,
  pub poll_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="user_invite_locks"]
pub struct NewUserInviteLock<'a> {
  pub user_invite_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="votes"]
pub struct NewVote<'a> {
  pub proposal_id: &'a i32,
  pub user_invite_id: &'a i32,
  pub points: &'a f64,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPollResource {
  pub point_totals: Option<HashMap<i32, f64>>,
  pub proposals: Option<Vec<Proposal>>,
  pub poll: Poll,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProposalPayload {
  pub summary: String,
  pub full_description_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVotePayload {
  pub points: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProposalResource {
  pub proposal: Proposal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericJsonResponse {
  pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeResource {
  pub polls: HashSet<Poll>,
  pub invite_poll_ids: Vec<i32>
}