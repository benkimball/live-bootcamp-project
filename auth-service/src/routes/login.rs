use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::AuthApiError, utils::auth::generate_auth_cookie};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, StatusCode), AuthApiError> {
    let user = state
        .user_store
        .read()
        .await
        .validate_user(&request.email.parse()?, &request.password.parse()?)
        .await
        .map_err(AuthApiError::from)?;
    let auth_cookie = generate_auth_cookie(&user.email).map_err(AuthApiError::from)?;
    let updated_jar = jar.add(auth_cookie);
    Ok((updated_jar, StatusCode::OK))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}
