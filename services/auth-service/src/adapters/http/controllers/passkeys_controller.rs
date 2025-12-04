use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::adapters::http::app_state::AppState;
use crate::application::use_cases::passkeys::{RegistrationResponse, AuthenticationResponse};

#[derive(Deserialize)]
pub struct StartRegistrationRequest {
    pub user_id: Uuid,
}

pub async fn start_registration(
    State(state): State<AppState>,
    Json(payload): Json<StartRegistrationRequest>,
) -> impl IntoResponse {
    match state.passkey_service.start_registration(payload.user_id).await {
        Ok(challenge) => (axum::http::StatusCode::OK, Json(challenge)).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to start registration: {}", e),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct FinishRegistrationRequest {
    pub user_id: Uuid,
    pub response: RegistrationResponse,
}

pub async fn finish_registration(
    State(state): State<AppState>,
    Json(payload): Json<FinishRegistrationRequest>,
) -> impl IntoResponse {
    match state.passkey_service.finish_registration(payload.user_id, payload.response).await {
        Ok(_passkey) => (axum::http::StatusCode::OK, "Registration successful").into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            format!("Failed to finish registration: {}", e),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct StartAuthenticationRequest {
    pub email: String,
}

pub async fn start_authentication(
    State(state): State<AppState>,
    Json(payload): Json<StartAuthenticationRequest>,
) -> impl IntoResponse {
    match state.passkey_service.start_authentication(&payload.email).await {
        Ok(challenge) => (axum::http::StatusCode::OK, Json(challenge)).into_response(),
        Err(e) => (
            axum::http::StatusCode::UNAUTHORIZED, // Could be "User not found" or "No passkeys"
            format!("Failed to start authentication: {}", e),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct FinishAuthenticationRequest {
    pub response: AuthenticationResponse,
}

pub async fn finish_authentication(
    State(state): State<AppState>,
    Json(payload): Json<FinishAuthenticationRequest>,
) -> impl IntoResponse {
    // TODO: Extract IP and User-Agent from headers
    let ip_address = None; // Placeholder
    let user_agent = None; // Placeholder

    match state.passkey_service.finish_authentication(payload.response, ip_address, user_agent).await {
        Ok((_user, session)) => (axum::http::StatusCode::OK, Json(session)).into_response(),
        Err(e) => (
            axum::http::StatusCode::UNAUTHORIZED,
            format!("Authentication failed: {}", e),
        )
            .into_response(),
    }
}
