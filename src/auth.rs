pub enum Auth {
    None,
    QueryPassword(User),
    SessionPassword(User),
    QueryToken(Token),
    SessionToken(Token),
}

pub struct Token {
    token: String,
    client_name: String,
}

pub struct User {
    username: String,
    password: String,
}
