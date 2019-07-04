use super::sql_enum_types;

table! {
    use super::super::sql_enum_types::Progress;
    use diesel::sql_types::*;

    polls (id) {
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
    proposals (id) {
        id -> Int4,
        summary -> Text,
        full_description_link -> Nullable<Varchar>,
        poll_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_invite_locks (id) {
        id -> Int4,
        user_invite_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    user_invites (id) {
        id -> Int4,
        email -> Varchar,
        poll_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    votes (id) {
        id -> Int4,
        user_invite_id -> Int4,
        proposal_id -> Int4,
        points -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(proposals -> polls (poll_id));
joinable!(user_invite_locks -> user_invites (user_invite_id));
joinable!(user_invites -> polls (poll_id));
joinable!(votes -> proposals (proposal_id));
joinable!(votes -> user_invites (user_invite_id));

allow_tables_to_appear_in_same_query!(
    polls,
    proposals,
    user_invite_locks,
    user_invites,
    votes,
);
