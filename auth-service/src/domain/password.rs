use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::ValidateLength;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Password(String);

impl FromStr for Password {
    type Err = super::AuthApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.validate_length(Some(8), None, None)
            .then_some(Password(s.to_string()))
            .ok_or(super::AuthApiError::InvalidCredentials)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::str::FromStr;

    #[quickcheck]
    fn prop_passwords_must_be_at_least_8_characters_long(password: String) -> bool {
        password.len() >= 8 || Password::from_str(&password).is_err()
    }

    #[test]
    fn test_password_equality() {
        let password1 = Password::from_str("password123").unwrap();
        let password2 = Password::from_str("password123").unwrap();
        let password3 = Password::from_str("different123").unwrap();

        assert_eq!(password1, password2);
        assert_ne!(password1, password3);
    }

    #[test]
    fn test_password_as_ref() {
        let password = Password::from_str("password123").unwrap();
        assert_eq!(password.as_ref(), "password123");
    }

    #[test]
    fn test_password_serialization() {
        let password = Password::from_str("password123").unwrap();

        // Test serialization
        let serialized = serde_json::to_string(&password).unwrap();
        assert_eq!(serialized, "\"password123\"");

        // Test deserialization
        let deserialized: Password = serde_json::from_str(&serialized).unwrap();
        assert_eq!(password, deserialized);
    }
}
