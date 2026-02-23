use crate::utils::auth::GenerateTokenError;

use super::UserStoreError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    #[default]
    UnexpectedError,
    MissingToken,
    InvalidToken,
}

impl From<UserStoreError> for AuthApiError {
    fn from(error: UserStoreError) -> Self {
        match error {
            UserStoreError::UserAlreadyExists => AuthApiError::UserAlreadyExists,
            UserStoreError::InvalidCredentials => AuthApiError::InvalidCredentials,
            UserStoreError::IncorrectCredentials => AuthApiError::IncorrectCredentials,
            _ => Default::default(),
        }
    }
}

impl From<GenerateTokenError> for AuthApiError {
    fn from(error: GenerateTokenError) -> Self {
        match error {
            GenerateTokenError::TokenError(_) => AuthApiError::InvalidToken,
            _ => Default::default(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthApiError {
    fn from(_error: jsonwebtoken::errors::Error) -> Self {
        AuthApiError::InvalidToken
    }
}
