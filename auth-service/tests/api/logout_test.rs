use crate::{refute, test_helpers::TestApp};
use auth_service::{domain::Token, utils::constants::JWT_COOKIE_NAME};
use reqwest::Url;

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").unwrap(),
    );
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}

// should return 200 if valid jwt cookie

#[tokio::test]
async fn should_return_200_and_ban_token_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    // first log in as valid user, to get the token from the response
    let response = app.create_user_and_log_in().await;
    let cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("JWT cookie not found");
    refute!(cookie.value().is_empty(), "Expected non-empty cookie value");
    let token = Token::from(cookie.value());
    eprintln!("Token: {}", token);

    // log out
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    // ensure that token is banned
    let store = app.banned_token_store.read().await;
    eprintln!("Store: {:?}", store);
    assert!(store.is_banned(&token).await);
}

// should return 400 if logout called twice in a row
#[tokio::test]
async fn should_return_400_if_logout_called_twice() {
    let app = TestApp::new().await;
    app.create_user_and_log_in().await;
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}
