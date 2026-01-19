#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    #[default]
    UnexpectedError,
}
