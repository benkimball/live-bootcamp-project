use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    Application,
};
use reqwest::cookie::Jar;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

// I don't like the look of `assert!(!...)`, hence:
/// Refute a condition, asserting that it is false.
#[macro_export]
macro_rules! refute {
    ($condition:expr) => {
        assert!(!$condition, "Expected condition to be false");
    };
    ($condition:expr, $msg:expr) => {
        assert!(!$condition, $msg);
    };
}

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let state = AppState::default();
        let banned_token_store = state.banned_token_store.clone();
        let two_fa_code_store = state.two_fa_code_store.clone();
        let app = Application::build(state, "127.0.0.1:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build client");
        Self {
            address,
            cookie_jar,
            http_client,
            banned_token_store,
            two_fa_code_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/signup", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/login", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn create_user_and_log_in(&self) -> reqwest::Response {
        let email = get_random_email();
        let signup_body = json!({
            "email": email,
            "password": "password123",
            "requires2FA": false,
        });
        let response = self.post_signup(&signup_body).await;
        assert_eq!(response.status().as_u16(), 201);

        let login_body = json!({
            "email": email,
            "password": "password123",
        });
        let response = self.post_login(&login_body).await;
        assert_eq!(response.status().as_u16(), 200);

        response
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}
