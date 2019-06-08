use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub email: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub picture: Option<String>,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub picture: Option<String>,
}
