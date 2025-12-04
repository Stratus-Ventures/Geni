use std::sync::Arc;
use crate::adapters::http::app_state::AppState;
use crate::infra::config::AppConfig;
use crate::application::use_cases::{
    users::UserService,
    passkeys::PasskeyService,
    magic_link::MagicLinkService,
    oauth::OAuthService,
    sessions::SessionService,
};

use crate::infra::repositories::postgres_auth_repository::PostgresAuthRepository;
use crate::infra::challenge_store::MemoryChallengeStore;
use crate::infra::webauthn::WebAuthnAdapter;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    
    // 1. Initialize Infrastructure (Adapters)
    // Connection pool
    let db_pool = sqlx::PgPool::connect(&config.database_url).await?;
    
    // Persistence
    let repo = Arc::new(PostgresAuthRepository::new(db_pool));
    
    // Stores & Crypto
    let challenge_store = Arc::new(MemoryChallengeStore::new());
    let webauthn_verifier = Arc::new(WebAuthnAdapter::new(&config));

    // 2. Initialize Application Layer (Use Cases)
    // Dependency Injection
    let user_service = Arc::new(UserService::new(repo.clone()));
    let passkey_service = Arc::new(PasskeyService::new(repo.clone(), challenge_store.clone(), webauthn_verifier.clone()));
    let magic_link_service = Arc::new(MagicLinkService::new(repo.clone()));
    let oauth_service = Arc::new(OAuthService::new(repo.clone()));
    let session_service = Arc::new(SessionService::new(repo.clone()));

    Ok(AppState {
        config: Arc::new(config),
        user_service,
        passkey_service,
        magic_link_service,
        oauth_service,
        session_service,
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "auth_service=debug,tower_http=debug,axum=debug".into());

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .pretty()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(true)
        )
        .init();
}
