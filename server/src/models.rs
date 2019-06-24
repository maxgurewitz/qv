use super::schema::ProgressEnum;
use chrono::prelude::*;

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
    pub updated_at: DateTime<Utc>
}