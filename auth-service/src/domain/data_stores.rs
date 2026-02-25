use crate::domain::{LoginAttemptId, TwoFACode};

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
pub trait BannedTokenStore: std::fmt::Debug + Send + Sync {
    async fn ban(&self, token: Token) -> BannedTokenResult;
    async fn is_banned(&self, token: &Token) -> bool;
    async fn unban(&self, token: &Token) -> BannedTokenResult;
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: std::fmt::Debug + Send + Sync {
    async fn add(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq, Default)]
pub enum TwoFACodeStoreError {
    #[default]
    EmailNotFound,
}
