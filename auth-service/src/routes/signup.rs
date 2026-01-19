use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::from(request);
    state.user_store.write().await.add_user(user).unwrap();
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    (StatusCode::CREATED, response)
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
