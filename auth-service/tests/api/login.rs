use auth_service::utils::constants::JWT_COOKIE_NAME;
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};

// 200 login successful
#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;
    let email = get_random_email();
    let signup_body = json!({
        "email": email,
        "password": "password123",
        "requires2FA": false,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = json!({
        "email": email,
        "password": "password123",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let response = app.post_login(&serde_json::json!({})).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_credentials() {
    let app = TestApp::new().await;
    let response = app
        .post_login(&serde_json::json!({ "email": "invalid", "password": "invalid" }))
        .await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let response = app
        .post_signup(&json!({
            "email": "user@example.com",
            "password": "password",
            "requires2FA": false,
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .post_login(&serde_json::json!({ "email": "user@example.com", "password": "incorrect" }))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}

// todo
// 206 login requires 2FA
// 500 unexpected error
