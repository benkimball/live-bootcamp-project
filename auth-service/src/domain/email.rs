use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl FromStr for Email {
    type Err = super::AuthApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.validate_email()
            .then_some(Email(s.to_string()))
            .ok_or(super::AuthApiError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_email_must_contain_at_symbol(email: String) -> bool {
        email.contains('@') || Email::from_str(&email).is_err()
    }

    #[quickcheck]
    fn prop_email_must_contain_dot(email: String) -> bool {
        email.contains('.') || Email::from_str(&email).is_err()
    }
}
