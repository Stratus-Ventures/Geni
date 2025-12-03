use std::sync::Arc;
use crate::application::use_cases::{
    users::UserService,
    passkeys::PasskeyService,
    magic_link::MagicLinkService,
    oauth::OAuthService,
    sessions::SessionService,
};
use crate::infra::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_service: Arc<UserService>,
    pub passkey_service: Arc<PasskeyService>,
    pub magic_link_service: Arc<MagicLinkService>,
    pub oauth_service: Arc<OAuthService>,
    pub session_service: Arc<SessionService>,
}
