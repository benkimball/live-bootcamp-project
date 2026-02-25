use std::str::FromStr;

use uuid::Uuid;

use crate::utils::auth::LoginAttemptIdError;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl FromStr for LoginAttemptId {
    type Err = LoginAttemptIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(String::from)
            .map(LoginAttemptId)
            .map_err(LoginAttemptIdError::InvalidUuid)
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(Uuid::new_v4().to_string())
    }
}
