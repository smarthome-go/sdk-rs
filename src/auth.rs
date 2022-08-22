use serde::Serialize;

pub enum Auth {
    None,
    QueryPassword(User),
    QueryToken(String),
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
