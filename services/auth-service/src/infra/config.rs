use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    
    // WebAuthn
    pub rp_origin: String,
    pub rp_id: String,
    pub rp_name: String,

    // OAuth - Google
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,

    // OAuth - Apple
    pub apple_client_id: String,
    pub apple_client_secret: String,
    pub apple_redirect_url: String,

    // Email
    pub resend_api_key: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid u16");

        // WebAuthn
        let rp_origin = env::var("RP_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let rp_id = env::var("RP_ID").unwrap_or_else(|_| "localhost".to_string());
        let rp_name = env::var("RP_NAME").unwrap_or_else(|_| "Auth Service".to_string());

        // OAuth - Google
        let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
        let google_redirect_url = env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set");

        // OAuth - Apple
        let apple_client_id = env::var("APPLE_CLIENT_ID").expect("APPLE_CLIENT_ID must be set");
        let apple_client_secret = env::var("APPLE_CLIENT_SECRET").expect("APPLE_CLIENT_SECRET must be set");
        let apple_redirect_url = env::var("APPLE_REDIRECT_URL").expect("APPLE_REDIRECT_URL must be set");

        // Email
        let resend_api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");

        Self {
            database_url,
            port,
            rp_origin,
            rp_id,
            rp_name,
            google_client_id,
            google_client_secret,
            google_redirect_url,
            apple_client_id,
            apple_client_secret,
            apple_redirect_url,
            resend_api_key,
        }
    }
}
