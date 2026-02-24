use super::{Email, Password, Token, User};

#[derive(Debug, PartialEq, Default)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    IncorrectCredentials,
    #[default]
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<User, UserStoreError>;
    async fn delete_user(&self, email: &Email) -> Result<User, UserStoreError>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BannedTokenResult {
    TokenAlreadyBanned,
    TokenBanned,
    TokenNotBanned,
    TokenUnbanned,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn ban(&self, token: Token) -> BannedTokenResult;
    async fn is_banned(&self, token: &Token) -> bool;
    async fn unban(&self, token: &Token) -> BannedTokenResult;
}
