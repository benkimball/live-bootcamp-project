use crate::{domain::AuthApiError, routes::*};
use app_state::AppState;
use axum::{
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

// This struct represents an API error that will be serialized to JSON.
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthApiError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthApiError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthApiError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthApiError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthApiError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthApiError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let ip = std::option_env!("DROPLET_IP").unwrap_or("localhost");
        let allowed_origins = [(format!("http://{}", ip)).parse()?];

        let cors = CorsLayer::new()
            .allow_origin(allowed_origins)
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true);

        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
