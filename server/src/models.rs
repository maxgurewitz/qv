#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Profile {
    pub email: String,
    // pub nonce: String,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub locale: Option<String>,
    pub picture: Option<String>,
}
