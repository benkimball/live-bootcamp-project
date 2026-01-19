use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    app_state::AppState,
    domain::{AuthApiError, User},
};

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    let user = User::try_from(request)?;
    state
        .user_store
        .add_user(user)
        .await
        .map_err(AuthApiError::from)?;
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
