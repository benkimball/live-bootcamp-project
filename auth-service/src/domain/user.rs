use crate::{routes::SignupRequest, AuthApiError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl TryFrom<SignupRequest> for User {
    type Error = AuthApiError;
    fn try_from(request: SignupRequest) -> Result<Self, Self::Error> {
        let email = request.email;
        let password = request.password;
        if email.is_empty() || !email.contains('@') || password.len() < 8 {
            return Err(AuthApiError::InvalidCredentials);
        }
        Ok(User::new(email, password, request.requires_2fa))
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
