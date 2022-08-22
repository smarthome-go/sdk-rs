pub enum Auth {
    None,
    QueryPassword(User),
    SessionPassword(User),
    QueryToken(String),
    SessionToken(String),
}

pub struct Token {
    token: String,
    client_name: Option<String>,
}

impl Token {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client_name: None,
        }
    }
    pub fn client_name(&self) -> Option<&str> {
        match &self.client_name {
            Some(n) => Some(&n),
            None => None,
        }
    }
}

pub struct User {
    pub username: String,
    pub password: String,
}
