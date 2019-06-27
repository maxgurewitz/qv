use super::sql_enum_types;

table! {
  use diesel::sql_types::*;
  use super::super::sql_enum_types::Progress;

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
