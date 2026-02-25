use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type TwoFACodeStoreType = Arc<RwLock<HashMap<Email, (LoginAttemptId, TwoFACode)>>>;

#[derive(Debug)]
pub struct HashMapTwoFACodeStore {
    codes: TwoFACodeStoreType,
}

impl Default for HashMapTwoFACodeStore {
    fn default() -> Self {
        Self {
            codes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {
    async fn add(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let mut codes = self.codes.write().await;
        codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let mut codes = self.codes.write().await;
        codes
            .remove(email)
            .map(|_| ())
            .ok_or(TwoFACodeStoreError::EmailNotFound)
    }

    async fn get(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let codes = self.codes.read().await;
        codes
            .get(email)
            .cloned()
            .ok_or(TwoFACodeStoreError::EmailNotFound)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    async fn get_test_fixture(
        login_attempt_ids: Vec<(Email, LoginAttemptId, TwoFACode)>,
    ) -> HashMapTwoFACodeStore {
        let mut store = HashMapTwoFACodeStore::default();
        for (email, attempt_id, code) in login_attempt_ids {
            store.add(email, attempt_id, code).await.unwrap();
        }
        store
    }

    #[tokio::test]
    async fn test_add_new_code_succeeds() {
        // if getting the test fixture when specifying at least one attempt ID succeeds,
        // that's sufficient
        let existing_code = (
            "test@example.com".parse().expect("valid email"),
            LoginAttemptId::default(),
            TwoFACode::from_str("123456").expect("valid 2FA code"),
        );
        get_test_fixture(vec![existing_code]).await;
    }

    #[tokio::test]
    async fn test_get_code_by_existing_email_succeeds() {
        let email: Email = "test@example.com".parse().expect("valid email");
        let attempt_id = LoginAttemptId::default();
        let code: TwoFACode = "123456".parse().expect("valid 2FA code");
        let store = get_test_fixture(vec![(email.clone(), attempt_id.clone(), code.clone())]).await;
        let record = store
            .get(&email)
            .await
            .expect("Failed to find existing code in store");
        assert_eq!(record.0, attempt_id);
        assert_eq!(record.1, code);
    }

    #[tokio::test]
    async fn test_get_by_nonexistent_email_fails() {
        let store = get_test_fixture(vec![]).await;
        let email: Email = "nope@example.com".parse().expect("valid email");
        assert_eq!(
            TwoFACodeStoreError::EmailNotFound,
            store
                .get(&email)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[tokio::test]
    async fn test_remove_by_existing_email_succeeds() {
        let email: Email = "test@example.com".parse().expect("valid email");
        let mut store = get_test_fixture(vec![(
            email.clone(),
            Default::default(),
            "123456".parse().expect("valid 2A code"),
        )])
        .await;
        assert!(store.remove(&email).await.is_ok());
    }

    #[tokio::test]
    async fn test_remove_by_nonexistent_email_fails() {
        let email: Email = "nope@example.com".parse().expect("valid email");
        let mut store = get_test_fixture(vec![]).await;
        assert_eq!(
            TwoFACodeStoreError::EmailNotFound,
            store
                .remove(&email)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }
}
