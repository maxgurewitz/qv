table! {
    users (email) {
        email -> Varchar,
        email_verified -> Nullable<Bool>,
        name -> Nullable<Varchar>,
        locale -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
    }
}
