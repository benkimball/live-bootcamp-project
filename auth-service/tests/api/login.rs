use serde_json::json;

use crate::helpers::TestApp;

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
// 200 login successful
// 206 login requires 2FA
// 500 unexpected error
