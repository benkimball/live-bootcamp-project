use dotenvy::dotenv;
use lazy_static::lazy_static;

pub const JWT_COOKIE_NAME: &str = "jwt";

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
}

fn set_token() -> String {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    if secret.is_empty() {
        panic!("JWT_SECRET cannot be empty");
    }
    secret
}
