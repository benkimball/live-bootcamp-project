use tokio::sync::RwLock;

use crate::{
    domain::{BannedTokenStore, UserStore},
    services::{HashMapUserStore, HashSetBannedTokenStore},
};
use std::sync::Arc;

// The Arc is necessary because trait objects (e.g., dyn UserStore) are
// dynamically-sized types (DSTs) whose size is not known at compile time.
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

/// Axum application state
/// Only includes a user store for now, will likely include more state in the future.
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokenStoreType) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}

/// Create a new AppState from a concrete user store.
impl<T: UserStore + 'static> From<T> for AppState {
    fn from(store: T) -> Self {
        Self::new(
            Arc::new(RwLock::new(store)),
            Arc::new(RwLock::new(HashSetBannedTokenStore::default())),
        )
    }
}

/// Create a new AppState defaulting to a HashMapUserStore user store.
impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: Arc::new(RwLock::new(HashMapUserStore::default())),
            banned_token_store: Arc::new(RwLock::new(HashSetBannedTokenStore::default())),
        }
    }
}
