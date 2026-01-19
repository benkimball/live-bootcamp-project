use super::UserStoreError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    #[default]
    UnexpectedError,
}

impl From<UserStoreError> for AuthApiError {
    fn from(error: UserStoreError) -> Self {
        match error {
            UserStoreError::UserAlreadyExists => AuthApiError::UserAlreadyExists,
            UserStoreError::InvalidCredentials => AuthApiError::InvalidCredentials,
            _ => Default::default(),
        }
    }
}
