use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub openid: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub openid: String,
    pub email: String,
}
