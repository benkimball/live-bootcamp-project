use tokio::sync::RwLock;

use crate::{domain::UserStore, services::HashMapUserStore};
use std::sync::Arc;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

/// Axum application state
/// Only includes a user store for now, will likely include more state in the future.
#[derive(Clone)]
pub struct AppState {
    // The Arc is necessary because trait objects (dyn UserStore) are
    // dynamically-sized types (DSTs) whose size is not known at compile time.
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}

/// Create a new AppState from a concrete user store.
impl<T: UserStore + 'static> From<T> for AppState {
    fn from(store: T) -> Self {
        Self::new(Arc::new(RwLock::new(store)))
    }
}

/// Create a new AppState defaulting to a HashMapUserStore user store.
impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: Arc::new(RwLock::new(HashMapUserStore::default())),
        }
    }
}
