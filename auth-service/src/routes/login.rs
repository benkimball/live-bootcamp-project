use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthApiError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, LoginResponse), AuthApiError> {
    let user = state
        .user_store
        .read()
        .await
        .validate_user(&request.email.parse()?, &request.password.parse()?)
        .await
        .map_err(AuthApiError::from)?;

    if user.requires_2fa {
        handle_2fa(State(state), &user.email, jar).await
    } else {
        handle_non_2fa(&user.email, jar).await
    }
}

async fn handle_2fa(
    State(state): State<AppState>,
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, LoginResponse), AuthApiError> {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();
    state
        .two_fa_code_store
        .write()
        .await
        .add(email.clone(), login_attempt_id, two_fa_code)
        .await
        .map_err(AuthApiError::from)?;
    Ok((
        jar,
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: String::from("2FA required"),
            login_attempt_id: String::from("123456"),
        }),
    ))
}

async fn handle_non_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, LoginResponse), AuthApiError> {
    let auth_cookie = generate_auth_cookie(email).map_err(AuthApiError::from)?;
    let updated_jar = jar.add(auth_cookie);
    Ok((updated_jar, LoginResponse::RegularAuth))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        match self {
            Self::RegularAuth => (StatusCode::OK, "Login successful").into_response(),
            Self::TwoFactorAuth(r) => (StatusCode::PARTIAL_CONTENT, Json(r)).into_response(),
        }
    }
}
