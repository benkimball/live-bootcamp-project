use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;
use serde_json::json;

#[tokio::test]
async fn signup_returns_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password",
            "requires2FA": true
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn should_return_409_if_user_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password",
            "requires2FA": true
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .post_signup(&json!({
            "email": random_email,
            "password": "password",
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
            "password": "password",
            "requires2FA": true
        }),
        &json!({
            "email": "",
            "password": "password",
            "requires2FA": true
        }),
        &json!({
            "email": "valid@example.com",
            "password": "short",
            "requires2FA": true
        }),
    ];
    for input in invalid_inputs {
        let response = app.post_signup(input).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            input
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_string()
        )
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let test_cases = [
        json!({
            "password": "password123",
            "requires2FA": true
        }),
        json!({
            "email": random_email,
            "password": "password",
            "password_confirmation": "password"
        }),
        json!({
            "email": "test@example.com",
            "password": "password",
        }),
        json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
