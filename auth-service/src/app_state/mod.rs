use crate::{domain::UserStore, services::HashMapUserStore};
use std::sync::Arc;

/// Axum application state
/// Only includes a user store for now, will likely include more state in the future.
pub struct AppState {
    // The Arc is necessary because trait objects (dyn UserStore) are
    // dynamically-sized types (DSTs) whose size is not known at compile time.
    pub user_store: Arc<dyn UserStore>,
}

impl AppState {
    pub fn new(user_store: Arc<dyn UserStore>) -> Self {
        Self { user_store }
    }
}

/// Create a new AppState from a concrete user store.
impl<T: UserStore + 'static> From<T> for AppState {
    fn from(store: T) -> Self {
        Self::new(Arc::new(store))
    }
}

/// Create a new AppState defaulting to a HashMapUserStore user store.
impl Default for AppState {
    fn default() -> Self {
        Self {
            user_store: Arc::new(HashMapUserStore::default()),
        }
    }
}
