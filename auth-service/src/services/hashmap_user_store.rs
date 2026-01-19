use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    #[default]
    UnexpectedError,
}

#[derive(Default, Debug)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
}

impl From<Vec<User>> for HashMapUserStore {
    fn from(users: Vec<User>) -> Self {
        let mut store = HashMapUserStore::default();
        for user in users {
            store.add_user(user).unwrap();
        }
        store
    }
}

impl HashMapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&mut self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if self.users.contains_key(email) {
            if let Some(user) = self.users.get(email) {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            } else {
                Err(UserStoreError::UserNotFound)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    #[allow(unused)]
    pub fn delete(&mut self, email: &str) -> Result<User, UserStoreError> {
        self.users.remove(email).ok_or(UserStoreError::UserNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_fixture() -> HashMapUserStore {
        HashMapUserStore::from(vec![
            User::new(
                String::from("test@example.com"),
                String::from("password"),
                false,
            ),
            User::new(
                String::from("test2@example.com"),
                String::from("password2"),
                true,
            ),
        ])
    }

    #[test]
    fn test_add_new_user_succeeds() {
        let mut store = get_test_fixture();
        let new_user = User::new(
            String::from("test3@example.com"),
            String::from("password"),
            false,
        );
        assert!(store.add_user(new_user).is_ok());
    }

    #[test]
    fn test_add_existing_user_fails() {
        let mut store = get_test_fixture();
        let new_user = User::new(
            String::from("test@example.com"),
            String::from("password"),
            false,
        );
        assert_eq!(
            UserStoreError::UserAlreadyExists,
            store
                .add_user(new_user)
                .expect_err("New user should already exist in fixture")
        );
    }

    #[test]
    fn test_get_user_by_existing_email_succeeds() {
        let store = get_test_fixture();
        let user = store
            .get_user("test@example.com")
            .expect("Test user should already exist in fixture");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_get_user_by_nonexistent_email_fails() {
        let store = get_test_fixture();
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .get_user("nope@example.com")
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[test]
    fn test_validate_unknown_user_fails() {
        let mut store = get_test_fixture();
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .validate_user("nope@example.com", "password")
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[test]
    fn test_validate_user_with_correct_credentials_succeeds() {
        let mut store = get_test_fixture();
        assert!(store.validate_user("test@example.com", "password").is_ok());
    }

    #[test]
    fn test_validate_user_with_incorrect_credentials_fails() {
        let mut store = get_test_fixture();
        assert_eq!(
            UserStoreError::InvalidCredentials,
            store
                .validate_user("test@example.com", "wrong_password")
                .expect_err("Test user should not exist in fixture")
        );
    }

    #[test]
    fn test_delete_user_by_existing_email_succeeds() {
        let mut store = get_test_fixture();
        assert!(store.delete("test@example.com").is_ok());
    }

    #[test]
    fn test_delete_user_by_nonexistent_email_fails() {
        let mut store = get_test_fixture();
        assert_eq!(
            UserStoreError::UserNotFound,
            store
                .delete("nope@example.com")
                .expect_err("Test user should not exist in fixture")
        );
    }
}
