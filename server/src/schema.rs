table! {
    poll (id) {
        id -> Int4,
        email -> Varchar,
        title -> Varchar,
        poll_type -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    poll_instance (id) {
        id -> Int4,
        poll_id -> Int4,
        has_started -> Bool,
        title -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    proposal (id) {
        id -> Int4,
        content -> Varchar,
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
    vote (user_invite_id) {
        user_invite_id -> Int4,
        points -> Nullable<Numeric>,
        created_at -> Timestamptz,
    }
}

joinable!(poll_instance -> poll (poll_id));
joinable!(proposal -> poll (poll_id));
joinable!(user_invite -> poll (poll_id));
joinable!(vote -> user_invite (user_invite_id));

allow_tables_to_appear_in_same_query!(
    poll,
    poll_instance,
    proposal,
    user_invite,
    users,
    vote,
);
