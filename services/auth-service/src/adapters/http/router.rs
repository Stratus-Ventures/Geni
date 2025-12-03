use axum::{
    Router, 
    routing::{post, get},
};
use crate::adapters::http::app_state::AppState;
use crate::adapters::http::controllers::{auth_controller, passkeys_controller, oauth_controller};
use crate::adapters::http::security::apply_security_layers;

pub fn api_router(state: AppState) -> Router {
    let magic_link_routes = Router::new()
        .route("/request", post(auth_controller::request_magic_link))
        .route("/verify", post(auth_controller::verify_magic_link));

    let passkey_routes = Router::new()
        .route("/register/start", post(passkeys_controller::start_registration))
        .route("/register/finish", post(passkeys_controller::finish_registration))
        .route("/login/start", post(passkeys_controller::start_authentication))
        .route("/login/finish", post(passkeys_controller::finish_authentication));

    let oauth_routes = Router::new()
        .route("/google", get(oauth_controller::google_login))
        .route("/google/callback", get(oauth_controller::google_callback))
        .route("/apple", get(oauth_controller::apple_login))
        .route("/apple/callback", post(oauth_controller::apple_callback));

    let auth_routes = Router::new()
        .nest("/magic-link", magic_link_routes)
        .nest("/passkey", passkey_routes)
        .nest("/oauth", oauth_routes);

    let api = Router::new()
        .nest("/auth", auth_routes)
        .with_state(state);
        
    apply_security_layers(api)
}
