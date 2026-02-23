use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::auth::generate_auth_token;
use serde_json::json;

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;
    let email = get_random_email().parse().expect("Failed to parse email");
    let body = json!({
        "token": generate_auth_token(&email).expect("Failed to generate auth token")
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let body = json!({
        "token": "nope"
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let body = json!({
        "nonsense": "foo"
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}
