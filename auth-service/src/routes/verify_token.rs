use crate::{
    app_state::AppState,
    domain::{AuthApiError, Token},
    utils::auth::validate_token,
};
use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde::Deserialize;

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthApiError> {
    if state
        .banned_token_store
        .read()
        .await
        .is_banned(&request.token)
        .await
    {
        return Err(AuthApiError::InvalidToken);
    }
    validate_token(&request.token).await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: Token,
}
