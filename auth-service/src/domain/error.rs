use super::UserStoreError;
use crate::{
    domain::TwoFACodeStoreError,
    utils::auth::{GenerateTokenError, LoginAttemptIdError},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    #[default]
    UnexpectedError,
    MissingToken,
    InvalidToken,
    InvalidTwoFaCode,
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

impl From<LoginAttemptIdError> for AuthApiError {
    fn from(_error: LoginAttemptIdError) -> Self {
        AuthApiError::InvalidTwoFaCode
    }
}

impl From<TwoFACodeStoreError> for AuthApiError {
    fn from(_error: TwoFACodeStoreError) -> Self {
        AuthApiError::UnexpectedError
    }
}
