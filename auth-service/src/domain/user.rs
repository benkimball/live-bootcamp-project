use super::{Email, Password};
use crate::{routes::SignupRequest, AuthApiError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl TryFrom<SignupRequest> for User {
    type Error = AuthApiError;
    fn try_from(request: SignupRequest) -> Result<Self, Self::Error> {
        let email = request.email.parse()?;
        let password = request.password.parse()?;
        Ok(User::new(email, password, request.requires_2fa))
    }
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::str::FromStr;

    #[quickcheck]
    fn prop_users_must_have_valid_email(email: String) -> bool {
        Email::from_str(&email).is_ok()
            || User::try_from(SignupRequest {
                email,
                password: "password".to_string(),
                requires_2fa: false,
            })
            .is_err()
    }

    #[quickcheck]
    fn prop_users_must_have_valid_password(password: String) -> bool {
        Password::from_str(&password).is_ok()
            || User::try_from(SignupRequest {
                email: "valid@email.com".to_string(),
                password,
                requires_2fa: false,
            })
            .is_err()
    }
}
