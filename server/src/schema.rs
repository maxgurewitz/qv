use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "progress")]
pub struct Progress;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "Progress"]
pub enum ProgressEnum {
    NotStarted,
    InProgress,
    Finished
}

// From example: https://github.com/ebkalderon/diesel/blob/db1a5156a7224ca978da806825efbfc3f349c558/diesel_tests/tests/custom_types.rs
impl ToSql<Progress, Pg> for ProgressEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            ProgressEnum::NotStarted => out.write_all(b"not_started")?,
            ProgressEnum::InProgress => out.write_all(b"in_progress")?,
            ProgressEnum::Finished => out.write_all(b"finished")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Progress, Pg> for ProgressEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"not_started" => Ok(ProgressEnum::NotStarted),
            b"in_progress" => Ok(ProgressEnum::InProgress),
            b"finished" => Ok(ProgressEnum::Finished),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

table! {
    use diesel::sql_types::*;
    use super::Progress;

    poll (id) {
        id -> Int4,
        email -> Varchar,
        title -> Varchar,
        poll_type -> Varchar,
        current_progress -> Progress,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    proposal (id) {
        id -> Int4,
        summary -> Text,
        full_description_link -> Nullable<Varchar>,
        poll_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_invite (id) {
        id -> Int4,
        email -> Varchar,
        poll_id -> Int4,
        done_voting -> Bool,
        created_at -> Timestamptz,
    }
}

table! {
    users (email) {
        email -> Varchar,
        email_verified -> Nullable<Bool>,
        name -> Nullable<Varchar>,
        locale -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
    }
}

table! {
    vote (id) {
        id -> Int4,
        user_invite_id -> Int4,
        proposal_id -> Int4,
        points -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(proposal -> poll (poll_id));
joinable!(user_invite -> poll (poll_id));
joinable!(vote -> proposal (proposal_id));
joinable!(vote -> user_invite (user_invite_id));

allow_tables_to_appear_in_same_query!(
    poll,
    proposal,
    user_invite,
    users,
    vote,
);
