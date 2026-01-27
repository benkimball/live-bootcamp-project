use super::UserStoreError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    #[default]
    UnexpectedError,
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
