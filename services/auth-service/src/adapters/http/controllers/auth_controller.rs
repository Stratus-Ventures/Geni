use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::adapters::http::app_state::AppState;

#[derive(Deserialize)]
pub struct RequestMagicLinkRequest {
    email: String,
}

pub async fn request_magic_link(
    State(state): State<AppState>,
    Json(payload): Json<RequestMagicLinkRequest>,
) -> impl IntoResponse {
    match state.magic_link_service.request_magic_link(&payload.email).await {
        Ok(token) => (axum::http::StatusCode::OK, Json(token)),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to send magic link".to_string())),
    }
}

#[derive(Deserialize)]
pub struct VerifyMagicLinkRequest {
    email: String,
    token: String,
}

pub async fn verify_magic_link(
    State(state): State<AppState>,
    Json(payload): Json<VerifyMagicLinkRequest>,
) -> impl IntoResponse {
    // Extract IP/User-Agent from headers in real impl
    match state.magic_link_service.verify_magic_link(&payload.email, &payload.token, None, None).await {
        Ok((_user, session)) => (axum::http::StatusCode::OK, Json(session)),
        Err(_) => (axum::http::StatusCode::UNAUTHORIZED, Json(crate::domain::models::Session { 
            id: "".to_string(), 
            user_id: uuid::Uuid::nil(), 
            expires_at: chrono::Utc::now(), 
            ip_address: None, 
            user_agent: None, 
            created_at: chrono::Utc::now() 
        })), // TODO: Better error response
    }
}
