use axum::{
    Router,
    http::{Method, header, StatusCode},
    error_handling::HandleErrorLayer,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    set_header::SetResponseHeaderLayer,
};
use axum::http::HeaderValue;
use tower::ServiceBuilder;
use std::sync::Arc;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

async fn handle_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", err),
    )
}

pub fn apply_security_layers(router: Router) -> Router {
    // 1. CORS Configuration
    // Allows specific domains to access the API
    // TODO: Configure specific origins for production
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]);

    // 2. Security Headers (XSS Protection, No Sniff)
    let secure_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_XSS_PROTECTION,
            HeaderValue::from_static("1; mode=block"),
        ));

    // 3. Rate Limiting (Governor)
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .use_headers() // Trust X-Forwarded-For (Be careful behind proxies!)
            .finish()
            .unwrap()
    );
    
    let rate_limit_layer = GovernorLayer::new(governor_conf);

    router
        // 4. Trace (Log everything) - Outermost
        .layer(TraceLayer::new_for_http())
        // 3. CORS (Handle preflight)
        .layer(cors)
        // 2. Governor + Error Handling
        // We apply HandleError AFTER Governor (so it wraps Governor)
        // Note: .layer() wraps the existing service.
        // router.layer(Gov).layer(HandleError) -> HandleError(Governor(Router))
        .layer(HandleErrorLayer::new(handle_error)) 
        .layer(rate_limit_layer)
        // 1. Headers (Inner)
        .layer(secure_headers)
}
