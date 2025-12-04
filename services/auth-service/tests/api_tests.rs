use axum::{
    body::Body,
    http::{Request, StatusCode},
    extract::ConnectInfo,
};
use tower::ServiceExt;
use std::sync::Arc;
use std::net::SocketAddr;
use auth_service::infra::config::AppConfig;
use auth_service::infra::repositories::mock::{MockAuthRepository, MockChallengeStore, MockWebAuthnVerifier};
use auth_service::application::use_cases::{
    users::UserService,
    passkeys::PasskeyService,
    magic_link::MagicLinkService,
    oauth::OAuthService,
    sessions::SessionService,
};
use auth_service::adapters::http::{router::api_router, app_state::AppState};
use serde_json::Value;

// Helper to setup the app with Mocks
async fn setup_app() -> axum::Router {
    // Config
    let config = AppConfig {
        database_url: "mock".to_string(),
        port: 3000,
        rp_origin: "http://localhost:3000".to_string(),
        rp_id: "localhost".to_string(),
        rp_name: "Mock Auth".to_string(),
        google_client_id: "mock".to_string(),
        google_client_secret: "mock".to_string(),
        google_redirect_url: "mock".to_string(),
        apple_client_id: "mock".to_string(),
        apple_client_secret: "mock".to_string(),
        apple_redirect_url: "mock".to_string(),
        resend_api_key: "mock".to_string(),
    };

    // Mocks
    let repo = Arc::new(MockAuthRepository::new());
    let challenge_store = Arc::new(MockChallengeStore::new());
    let webauthn_verifier = Arc::new(MockWebAuthnVerifier::new());

    // Services
    let user_service = Arc::new(UserService::new(repo.clone()));
    let passkey_service = Arc::new(PasskeyService::new(repo.clone(), challenge_store.clone(), webauthn_verifier.clone()));
    let magic_link_service = Arc::new(MagicLinkService::new(repo.clone()));
    let oauth_service = Arc::new(OAuthService::new(repo.clone()));
    let session_service = Arc::new(SessionService::new(repo.clone()));

    let state = AppState {
        config: Arc::new(config),
        user_service,
        passkey_service,
        magic_link_service,
        oauth_service,
        session_service,
    };

    api_router(state)
}

fn build_req(uri: &str, method: &str, body: Body) -> Request<Body> {
    let mut req = Request::builder()
        .uri(uri)
        .method(method)
        .header("content-type", "application/json")
        .body(body)
        .unwrap();
    
    // Mock IP for Rate Limiting
    let addr = SocketAddr::from(([127, 0, 0, 1], 12345));
    req.extensions_mut().insert(ConnectInfo(addr));
    
    req
}

#[tokio::test]
async fn test_health_check_not_found() {
    let app = setup_app().await;

    let req = build_req("/health", "GET", Body::empty());
    let response = app.oneshot(req).await.unwrap();

    // We didn't define /health, so should be 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_security_headers() {
    let app = setup_app().await;

    // Hit any route (e.g. 404) to check global middleware
    let req = build_req("/random", "GET", Body::empty());
    let response = app.oneshot(req).await.unwrap();

    let headers = response.headers();
    assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
    assert_eq!(headers.get("x-xss-protection").unwrap(), "1; mode=block");
}

#[tokio::test]
async fn test_magic_link_flow() {
    let app = setup_app().await;

    // 1. Request Link
    let req_body = serde_json::json!({ "email": "test@example.com" });
    let req = build_req("/auth/magic-link/request", "POST", Body::from(req_body.to_string()));
    
    let response = app.clone().oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let token_str: String = serde_json::from_slice(&body_bytes).unwrap();
    println!("Received Token: {}", token_str);

    // 2. Verify Link
    let verify_body = serde_json::json!({ 
        "email": "test@example.com",
        "token": token_str 
    });
    
    let req_verify = build_req("/auth/magic-link/verify", "POST", Body::from(verify_body.to_string()));
    
    let response = app.oneshot(req_verify).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let session: Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert!(session.get("id").is_some());
    assert!(session.get("user_id").is_some());
}

#[tokio::test]
async fn test_rate_limiting() {
    let app = setup_app().await;

    // Send 15 requests (limit is 10/sec, burst 20). 
    // Burst 20 means we can do 20 instantly. We need to exceed burst.
    // Configuration in security.rs: per_second(10), burst_size(20).
    // We need > 20 requests to trigger 429.
    
    let mut hit_limit = false;
    
    for _ in 0..30 {
        let req = build_req("/random", "GET", Body::empty());
        // Clone app for each request (ServiceExt consumes it)
        let res = app.clone().oneshot(req).await.unwrap();
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            hit_limit = true;
            break;
        }
    }

    // Note: Governor is inconsistent in tests due to timer/clock mocks, 
    // but usually works if we blast enough requests.
    assert!(hit_limit, "Should have hit rate limit");
}

#[tokio::test]
async fn test_oauth_redirect() {
    let app = setup_app().await;

    let req = build_req("/auth/oauth/google", "GET", Body::empty());
    let response = app.oneshot(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER); // 303 Redirect
    let location = response.headers().get("location").unwrap().to_str().unwrap();
    assert!(location.contains("accounts.google.com"));
    assert!(location.contains("client_id=mock"));
}

