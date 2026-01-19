use crate::routes::SignupRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl From<SignupRequest> for User {
    fn from(request: SignupRequest) -> Self {
        User {
            email: request.email,
            password: request.password,
            requires_2fa: request.requires_2fa,
        }
    }
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}
