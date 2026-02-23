use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{
    domain::AuthApiError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, StatusCode), AuthApiError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthApiError::MissingToken)?;
    let token = cookie.value().to_owned();
    let _claims = validate_token(&token)
        .await
        .map_err(|_| AuthApiError::InvalidToken)?;
    // remove JWT cookie from the CookieJar
    Ok((jar.remove(JWT_COOKIE_NAME), StatusCode::OK))
}
