pub struct Client {
    auth: Auth,
    smarthome_url: url::Url,
    smarthome_version: String,
}

pub enum Auth {
    None,
    QueryPassword(User),
    SessionPassword(User),
    QueryToken(Token),
    SessionToken(Token),
}

struct Token {
    token: String,
    client_name: String,
}

struct User{
    username: String,
    password: String
}

