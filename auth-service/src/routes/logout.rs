use axum::extract::State;
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{
    app_state::AppState,
    domain::{AuthApiError, Token},
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), AuthApiError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthApiError::MissingToken)?;
    let token = Token::from(cookie.value());
    let _claims = validate_token(&token)
        .await
        .map_err(|_| AuthApiError::InvalidToken)?;
    // remove JWT cookie from the CookieJar
    let jar = jar.remove(JWT_COOKIE_NAME);
    // add token to the banned list
    state.banned_token_store.write().await.ban(token).await;
    Ok((jar, StatusCode::OK))
}
