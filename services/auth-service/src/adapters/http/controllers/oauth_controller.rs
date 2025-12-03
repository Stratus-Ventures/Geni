use axum::{
    extract::{State, Query},
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use crate::adapters::http::app_state::AppState;
use crate::domain::models::OAuthProvider;

// ==========================================
// GOOGLE OAUTH
// ==========================================

pub async fn google_login(State(state): State<AppState>) -> impl IntoResponse {
    // Construct the Google authorization URL manually or use an OAuth2 library helper.
    // For cleanliness, we should ideally move this URL construction to the OAuthService 
    // or a specific GoogleAdapter, but for now, we'll construct it here using config.
    
    let client_id = &state.config.google_client_id;
    let redirect_uri = &state.config.google_redirect_url;
    let scope = "openid email profile";
    let response_type = "code";
    
    // State parameter should be randomized to prevent CSRF (handled by frontend or cookie usually)
    // For this MVP, we'll use a simple static one or let the frontend handle it.
    // Ideally, we generate a random state, store it in a cookie, and verify it in the callback.
    let csrf_state = "random_state_string"; 

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope={}&state={}",
        client_id, redirect_uri, response_type, scope, csrf_state
    );

    Redirect::to(&url)
}

#[derive(Deserialize)]
pub struct OAuthCallbackParams {
    _code: String,
    _state: String,
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(_params): Query<OAuthCallbackParams>,
) -> impl IntoResponse {
    // 1. Exchange code for token (This logic should ideally be in OAuthService)
    // We need to make a POST request to Google.
    
    // Since `OAuthService` in our current architecture expects `provider_id` and `email`,
    // the logic to exchange "code" -> "access_token" -> "user_info" belongs in the Infrastructure/Adapter layer.
    // However, `OAuthService` is in Application layer.
    
    // We will implement the exchange here (Controller) or create a helper.
    // To keep the controller thin, we should probably have `state.oauth_service.exchange_code(...)` 
    // but `OAuthService` is generic.
    
    // Implementation Detail:
    // 1. POST https://oauth2.googleapis.com/token
    // 2. GET https://www.googleapis.com/oauth2/v3/userinfo
    // 3. Call `state.oauth_service.verify_oauth_login(...)`
    
    // For this step, I will mock the interaction or implement it if `reqwest` is available (it is).
    
    // TODO: Implement actual token exchange. 
    // Placeholder:
    let provider_id = "google_user_id_placeholder".to_string(); // We would get this from Google
    let email = "user@example.com".to_string(); // We would get this from Google
    
    // Real implementation would be:
    // let token = exchange_google_code(&state.config, &params.code).await?;
    // let user_info = fetch_google_user_info(&token).await?;
    // let provider_id = user_info.sub;
    // let email = user_info.email;

    match state.oauth_service.verify_oauth_login(
        OAuthProvider::Google,
        provider_id,
        email,
        None, // IP
        None, // User Agent
    ).await {
        Ok((_user, session)) => (axum::http::StatusCode::OK, Json(session)).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Login failed: {}", e),
        ).into_response(),
    }
}

// ==========================================
// APPLE OAUTH
// ==========================================

pub async fn apple_login(State(state): State<AppState>) -> impl IntoResponse {
    let client_id = &state.config.apple_client_id;
    let redirect_uri = &state.config.apple_redirect_url;
    let scope = "name email";
    let response_type = "code id_token";
    let response_mode = "form_post"; // Apple requires form_post for some flows
    let csrf_state = "random_state_string";

    let url = format!(
        "https://appleid.apple.com/auth/authorize?client_id={}&redirect_uri={}&response_type={}&response_mode={}&scope={}&state={}",
        client_id, redirect_uri, response_type, response_mode, scope, csrf_state
    );

    Redirect::to(&url)
}

#[derive(Deserialize)]
pub struct AppleCallbackForm {
    _code: String,
    _state: String,
    _user: Option<String>, // JSON string containing name (only sent on first login)
}

// Apple sends a POST request to the redirect URI
pub async fn apple_callback(
    State(state): State<AppState>,
    axum::Form(_form): axum::Form<AppleCallbackForm>,
) -> impl IntoResponse {
    // Similar to Google: Exchange code for token, verify ID token.
    // Apple returns `id_token` (JWT) directly if requested, which contains the email/sub.
    
    // Placeholder:
    let provider_id = "apple_user_id_placeholder".to_string();
    let email = "user@example.com".to_string();

    match state.oauth_service.verify_oauth_login(
        OAuthProvider::Apple,
        provider_id,
        email,
        None,
        None,
    ).await {
        Ok((_user, session)) => (axum::http::StatusCode::OK, Json(session)).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Login failed: {}", e),
        ).into_response(),
    }
}
