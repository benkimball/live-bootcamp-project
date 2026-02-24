use crate::domain::{BannedTokenResult, BannedTokenStore, Token};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

type BannedTokenStoreType = Arc<RwLock<HashSet<Token>>>;

#[derive(Debug)]
pub struct HashSetBannedTokenStore {
    tokens: BannedTokenStoreType,
}

impl Default for HashSetBannedTokenStore {
    fn default() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

// Just a convenience, for tests

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn ban(&self, token: Token) -> BannedTokenResult {
        let mut tokens = self.tokens.write().await;
        if tokens.contains(&token) {
            BannedTokenResult::TokenAlreadyBanned
        } else {
            tokens.insert(token);
            BannedTokenResult::TokenBanned
        }
    }

    async fn is_banned(&self, token: &Token) -> bool {
        let tokens = self.tokens.read().await;
        tokens.contains(token)
    }

    async fn unban(&self, token: &Token) -> BannedTokenResult {
        let mut tokens = self.tokens.write().await;
        if tokens.remove(token) {
            BannedTokenResult::TokenUnbanned
        } else {
            BannedTokenResult::TokenNotBanned
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a prepopulated banned user store, for convenience in testing
    impl<S: AsRef<str>> FromIterator<S> for HashSetBannedTokenStore {
        fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
            let tokens = Arc::new(RwLock::new(
                iter.into_iter()
                    .map(|item| Token::from(item.as_ref()))
                    .collect::<HashSet<Token>>(),
            ));
            Self { tokens }
        }
    }

    #[tokio::test]
    async fn test_ban() {
        let store = HashSetBannedTokenStore::from_iter(vec!["bad", "verybad"]);
        let token = Token::from("superbad");
        assert_eq!(
            store.ban(token.clone()).await,
            BannedTokenResult::TokenBanned
        );
        assert!(store.is_banned(&token).await);
    }

    #[tokio::test]
    async fn test_ban_already_banned() {
        let store = HashSetBannedTokenStore::from_iter(vec!["bad", "verybad"]);
        let token = Token::from("bad");
        assert_eq!(
            store.ban(token.clone()).await,
            BannedTokenResult::TokenAlreadyBanned
        );
        assert!(store.is_banned(&token).await);
    }

    #[tokio::test]
    async fn test_unban() {
        let store = HashSetBannedTokenStore::from_iter(vec!["bad", "verybad"]);
        let token = Token::from("bad");
        assert_eq!(store.unban(&token).await, BannedTokenResult::TokenUnbanned);
        assert!(!store.is_banned(&token).await);
    }

    #[tokio::test]
    async fn test_unban_not_banned() {
        let store = HashSetBannedTokenStore::from_iter(vec!["bad", "verybad"]);
        let token = Token::from("superbad");
        assert_eq!(store.unban(&token).await, BannedTokenResult::TokenNotBanned);
        assert!(!store.is_banned(&token).await);
    }
}
