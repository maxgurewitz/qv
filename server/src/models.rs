use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub openid: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    // pub openid: &'a str,
    pub openid: String,
    pub email: &'a str,
}
