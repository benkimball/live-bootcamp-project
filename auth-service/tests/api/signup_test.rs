use crate::test_helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;
use serde_json::json;

macro_rules! assert_all_status {
    ($collection:expr, $func:expr, $status:expr) => {
        for item in $collection {
            let result = $func(item).await;
            assert!(
                result.status().as_u16() == $status,
                "Assertion failed for input: '{:?}' - result: {:?}",
                item,
                result
            );
        }
    };
}

#[tokio::test]
async fn signup_returns_201_with_valid_input() {
    let app = TestApp::new().await;
    let valid_inputs = [
        &json!({
            "email": "normal@example.com",
            "password": "password123",
            "requires2FA": true
        }),
        &json!({
            "email": "crazylong@password.com",
            "password": "a".repeat(1000),
            "requires2FA": true
        }),
    ];
    assert_all_status!(valid_inputs, |input| app.post_signup(input), 201);
}

#[tokio::test]
async fn should_return_409_if_user_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        }))
        .await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_string()
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let invalid_inputs = [
        &json!({
            "email": "invalid_email",
            "password": "password123",
            "requires2FA": true
        }),
        &json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        &json!({
            "email": "valid@example.com",
            "password": "short",
            "requires2FA": true
        }),
        &json!({
            "email": "~välød@utf8.com",
            "password": "password123",
            "requires2FA": true
        }),
    ];
    assert_all_status!(invalid_inputs, |input| app.post_signup(input), 400);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let test_cases = [
        &json!({
            "password": "password123",
            "requires2FA": true
        }),
        &json!({
            "email": random_email,
            "password": "password123",
            "password_confirmation": "password123"
        }),
        &json!({
            "email": "test@example.com",
            "password": "password123",
        }),
        &json!({}),
    ];
    assert_all_status!(test_cases, |input| app.post_signup(input), 422);
}
