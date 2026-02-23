use crate::{domain::AuthApiError, utils::auth::validate_token};
use axum::Json;
use reqwest::StatusCode;
use serde::Deserialize;

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthApiError> {
    validate_token(&request.token).await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
