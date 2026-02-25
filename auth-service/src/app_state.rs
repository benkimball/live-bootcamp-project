use tokio::sync::RwLock;

use crate::{
    domain::{BannedTokenStore, TwoFACodeStore, UserStore},
    services::{HashMapTwoFACodeStore, HashMapUserStore, HashSetBannedTokenStore},
};
use std::sync::Arc;

// The Arc is necessary because trait objects (e.g., dyn UserStore) are
// dynamically-sized types (DSTs) whose size is not known at compile time.
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore + Send + Sync>>;

/// Axum application state
/// Only includes a user store for now, will likely include more state in the future.
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_token_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
        }
    }
}

/// Create a new AppState defaulting to a HashMapUserStore user store.
impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: Arc::new(RwLock::new(HashMapUserStore::default())),
            banned_token_store: Arc::new(RwLock::new(HashSetBannedTokenStore::default())),
            two_fa_code_store: Arc::new(RwLock::new(HashMapTwoFACodeStore::default())),
        }
    }
}
