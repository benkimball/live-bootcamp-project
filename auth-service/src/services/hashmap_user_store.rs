use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type UserStoreType = Arc<RwLock<HashMap<Email, User>>>;

#[derive(Debug)]
pub struct HashMapUserStore {
    users: UserStoreType,
}

impl Default for HashMapUserStore {
    fn default() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
        let mut users = self.users.write().await;
        if users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let users = self.users.read().await;
        users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let users = self.users.read().await;
        if let Some(user) = users.get(email) {
            if user.password == *password {
                Ok(())
            } else {
                Err(UserStoreError::IncorrectCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn delete_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let mut users = self.users.write().await;
        users.remove(email).ok_or(UserStoreError::UserNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_fixture() -> HashMapUserStore {
        let store = HashMapUserStore::default();
        store
            .add_user(User::new(
                "test@example.com".parse().expect("valid email"),
                "password123".parse().expect("valid password"),
                false,
            ))
            .await
            .unwrap();
        store
            .add_user(User::new(
                "test2@example.com".parse().expect("valid email"),
                "password234".parse().expect("valid password"),
                true,
            ))
            .await
            .unwrap();
        store
    }

    #[tokio::test]
    async fn test_add_new_user_succeeds() {
        let store = get_test_fixture().await;
        let new_user = User::new(
            "test3@example.com".parse().expect("valid email"),
            "password345".parse().expect("valid password"),
            false,
        );
        assert!(store.add_user(new_user).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_existing_user_fails() {
        let store = get_test_fixture().await;
        let new_user = User::new(
            "test@example.com".parse().expect("valid email"),
            "password123".parse().expect("valid password"),
            false,
        );
        assert_eq!(
            UserStoreError::UserAlreadyExists,
            store
                .add_user(new_user)
                .await
                .expect_err("New user should already exist in fixture")
        );
    }

    #[tokio::test]
    async fn test_get_user_by_existing_email_succeeds() {
        let store = get_test_fixture().await;
        let email: Email = "test@example.com".parse().expect("valid email");
        let user = store
            .get_user(&email)
            .await
            .expect("Test user should already exist in fixture");
        assert_eq!(user.email, email);
    }

    #[tokio::test]
    async fn test_get_user_by_nonexistent_email_fails() {
        let store = get_test_fixture().await;
        let email: Email = "nope@example.com".parse().expect("valid email");
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .get_user(&email)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[tokio::test]
    async fn test_validate_unknown_user_fails() {
        let store = get_test_fixture().await;
        let email: Email = "nope@example.com".parse().expect("valid email");
        let password: Password = "password123".parse().expect("valid password");
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .validate_user(&email, &password)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[tokio::test]
    async fn test_validate_user_with_correct_credentials_succeeds() {
        let store = get_test_fixture().await;
        let email: Email = "test@example.com".parse().expect("valid email");
        let password: Password = "password123".parse().expect("valid password");
        assert!(store.validate_user(&email, &password).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_with_incorrect_credentials_fails() {
        let store = get_test_fixture().await;
        let email: Email = "test@example.com".parse().expect("valid email");
        let wrong_password: Password = "wrong_password_long".parse().expect("valid password");
        assert_eq!(
            UserStoreError::IncorrectCredentials,
            store
                .validate_user(&email, &wrong_password)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[tokio::test]
    async fn test_delete_user_by_existing_email_succeeds() {
        let store = get_test_fixture().await;
        let email: Email = "test@example.com".parse().expect("valid email");
        assert!(store.delete_user(&email).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_user_by_nonexistent_email_fails() {
        let store = get_test_fixture().await;
        let email: Email = "nope@example.com".parse().expect("valid email");
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .delete_user(&email)
                .await
                .expect_err("Test user should not exist in fixture")
        );
    }
}
