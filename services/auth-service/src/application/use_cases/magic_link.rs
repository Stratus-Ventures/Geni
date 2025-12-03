use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};
use sha2::{Sha256, Digest};

use crate::application::app_error::{AppError, AppResult};
use crate::domain::models::*;
use crate::domain::ports::AuthRepository;

/// The MagicLinkService handles email-based passwordless authentication.
/// Users receive a one-time link via email that logs them in.
pub struct MagicLinkService {
    repo: Arc<dyn AuthRepository>,
}

impl MagicLinkService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    /// Request a magic link for the given email address.
    ///
    /// **Flow:**
    /// 1. Validate the email format.
    /// 2. Generate a cryptographically random token (32 bytes = 64 hex chars).
    /// 3. Hash the token with SHA256 (store only the hash, never the raw token).
    /// 4. Save the hash to the database with a 15-minute expiration.
    /// 5. Return the raw token so it can be sent via email.
    ///
    /// **Security Note:** The raw token is only returned here for the infrastructure layer
    /// to send via email. It's NEVER stored in the database.
    pub async fn request_magic_link(&self, email: &str) -> AppResult<String> {
        // Validate email format
        if !is_valid_email(email) {
            return Err(AppError::InvalidCredentials);
        }

        // Normalize email to lowercase for consistency
        let normalized_email = email.to_lowercase();
        // Generate a random 32-byte token (64 hex characters)
        let raw_token = generate_random_token(32);
        // Hash the token with SHA256
        let token_hash = hash_token(&raw_token);
        // Create the magic link record with 15-minute expiration
        let magic_link = MagicLinkToken {
            email: normalized_email.clone(),
            token_hash,
            expires_at: Utc::now() + Duration::minutes(5),
            used: false,
        };

        // Save to database via repository
        self.repo
            .save_magic_link(magic_link)
            .await?;

        Ok(raw_token)
    }

    /// Verify a magic link token and create a session.
    ///
    /// **Flow:**
    /// 1. Hash the incoming token and compare with the stored hash.
    /// 2. Verify the token hasn't expired.
    /// 3. Verify the token hasn't been used already (one-time use).
    /// 4. Find the user by email, or create a new user if it's their first time.
    /// 5. Mark the token as used.
    /// 6. Create and return a new session.
    pub async fn verify_magic_link(
        &self,
        email: &str,
        token: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<(User, Session)> {
        let normalized_email = email.to_lowercase();

        // Find the magic link record
        let magic_link = self
            .repo
            .find_magic_link_by_email(&normalized_email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify the token hasn't been used
        if magic_link.used {
            return Err(AppError::InvalidCredentials);
        }

        // Verify the token hasn't expired
        if magic_link.expires_at < Utc::now() {
            return Err(AppError::InvalidCredentials);
        }

        // Hash the incoming token and compare with stored hash
        let token_hash = hash_token(token);
        if token_hash != magic_link.token_hash {
            return Err(AppError::InvalidCredentials);
        }

        // Mark the token as used (one-time use only)
        self.repo
            .mark_magic_link_as_used(&normalized_email)
            .await?;

        // Find existing user or create a new one
        let user = match self.repo.find_user_by_email(&normalized_email).await {
            Ok(Some(existing_user)) => {
                // Update last sign-in information
                let mut updated_user = existing_user;
                updated_user.last_sign_in = Some(Utc::now());
                updated_user.last_sign_in_method = Some(LastSignInMethod::MagicLink);
                updated_user.updated_at = Utc::now();

                self.repo
                    .update_user(updated_user.clone())
                    .await?;

                updated_user
            }
            Ok(None) => {
                // New user - create account
                let new_user = User {
                    id: Uuid::new_v4(),
                    name: normalized_email.split('@').next().unwrap_or("User").to_string(),
                    email: normalized_email.clone(),
                    phone: None,
                    plan: UserPlan::FreeTrial,
                    last_sign_in: Some(Utc::now()),
                    last_sign_in_method: Some(LastSignInMethod::MagicLink),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                self.repo
                    .create_user(new_user.clone())
                    .await?;

                new_user
            }
            Err(e) => {
                return Err(AppError::from(e));
            }
        };

        // Create a new session (30 days)
        let session = Session {
            id: generate_session_id(),
            user_id: user.id,
            expires_at: Utc::now() + Duration::days(30),
            ip_address,
            user_agent,
            created_at: Utc::now(),
        };

        let created_session = self
            .repo
            .create_session(session)
            .await?;

        Ok((user, created_session))
    }
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Basic email validation (RFC 5322 simplified).
/// For production, consider using the `validator` or `email_address` crate.
fn is_valid_email(email: &str) -> bool {
    if email.len() < 3 {
        return false;
    }

    // Must contain exactly one '@'
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    // Local part (before @) must not be empty
    if local.is_empty() {
        return false;
    }

    // Domain part (after @) must contain at least one '.' and not be empty
    if domain.is_empty() || !domain.contains('.') {
        return false;
    }

    // Domain must not start or end with '.'
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    true
}

/// Generate a cryptographically secure random token as a hex string.
fn generate_random_token(byte_length: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..byte_length).map(|_| rng.r#gen()).collect();
    hex::encode(bytes)
}

/// Hash a token using SHA256 and return as hex string.
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Generate a secure session ID (32 random bytes = 64 hex characters).
fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.r#gen()).collect();
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use async_trait::async_trait;
    use crate::domain::errors::RepositoryError;

    // Mock Repository
    struct MockAuthRepository {
        users: Mutex<HashMap<Uuid, User>>,
        users_by_email: Mutex<HashMap<String, Uuid>>,
        magic_links: Mutex<HashMap<String, MagicLinkToken>>, // Keyed by email
        sessions: Mutex<HashMap<String, Session>>,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
                users_by_email: Mutex::new(HashMap::new()),
                magic_links: Mutex::new(HashMap::new()),
                sessions: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl AuthRepository for MockAuthRepository {
        async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
            let users = self.users.lock().unwrap();
            Ok(users.get(&id).cloned())
        }

        async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
            let map = self.users_by_email.lock().unwrap();
            if let Some(id) = map.get(email) {
                let users = self.users.lock().unwrap();
                Ok(users.get(id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn create_user(&self, user: User) -> Result<User, RepositoryError> {
            let mut users = self.users.lock().unwrap();
            let mut by_email = self.users_by_email.lock().unwrap();
            
            if by_email.contains_key(&user.email) {
                return Err(RepositoryError::DuplicateKey("email".to_string()));
            }
            
            by_email.insert(user.email.clone(), user.id);
            users.insert(user.id, user.clone());
            Ok(user)
        }

        async fn update_user(&self, user: User) -> Result<User, RepositoryError> {
            let mut users = self.users.lock().unwrap();
            if users.contains_key(&user.id) {
                users.insert(user.id, user.clone());
                Ok(user)
            } else {
                Err(RepositoryError::NotFound("User".to_string()))
            }
        }

        async fn delete_user(&self, id: Uuid) -> Result<(), RepositoryError> {
            let mut users = self.users.lock().unwrap();
            let mut by_email = self.users_by_email.lock().unwrap();
            
            if let Some(user) = users.remove(&id) {
                by_email.remove(&user.email);
            }
            Ok(())
        }

        // Magic Link methods
        async fn save_magic_link(&self, token: MagicLinkToken) -> Result<(), RepositoryError> {
            let mut links = self.magic_links.lock().unwrap();
            links.insert(token.email.clone(), token);
            Ok(())
        }

        async fn find_magic_link_by_email(&self, email: &str) -> Result<Option<MagicLinkToken>, RepositoryError> {
            let links = self.magic_links.lock().unwrap();
            Ok(links.get(email).cloned())
        }

        async fn mark_magic_link_as_used(&self, email: &str) -> Result<(), RepositoryError> {
            let mut links = self.magic_links.lock().unwrap();
            if let Some(link) = links.get_mut(email) {
                link.used = true;
                Ok(())
            } else {
                Err(RepositoryError::NotFound("MagicLink".to_string()))
            }
        }

        async fn delete_magic_links_for_email(&self, email: &str) -> Result<(), RepositoryError> {
            let mut links = self.magic_links.lock().unwrap();
            links.remove(email);
            Ok(())
        }
        
        async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> { Ok(()) }

        // Session methods
        async fn create_session(&self, session: Session) -> Result<Session, RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        async fn find_session_by_id(&self, token: &str) -> Result<Option<Session>, RepositoryError> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.get(token).cloned())
        }

        async fn delete_session(&self, token: &str) -> Result<(), RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(token);
            Ok(())
        }

        async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.retain(|_, s| s.user_id != user_id);
            Ok(())
        }
        
        async fn update_session(&self, session: Session) -> Result<Session, RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            if sessions.contains_key(&session.id) {
                sessions.insert(session.id.clone(), session.clone());
                Ok(session)
            } else {
                Err(RepositoryError::NotFound("Session".to_string()))
            }
        }
        
        async fn delete_expired_sessions(&self) -> Result<(), RepositoryError> { Ok(()) }

        // Unused methods for this test suite
        async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError> { Ok(passkey) }
        async fn find_passkey_by_credential_id(&self, _id: &str) -> Result<Option<Passkey>, RepositoryError> { Ok(None) }
        async fn find_passkeys_by_user_id(&self, _user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError> { Ok(vec![]) }
        async fn update_passkey_sign_count(&self, _credential_id: &str, _count: i64) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_passkeys_for_user(&self, _user_id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        
        async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, RepositoryError> { Ok(identity) }
        async fn find_user_identity(&self, _provider: &OAuthProvider, _provider_id: &str) -> Result<Option<UserIdentity>, RepositoryError> { Ok(None) }
        async fn delete_user_identities_for_user(&self, _user_id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
    }

    #[tokio::test]
    async fn test_request_magic_link() {
        dbg!("\n✨ TEST: test_request_magic_link ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = MagicLinkService::new(repo.clone());

        // Test valid email
        let email = "test@example.com";
        dbg!("  ➡️  Action: Requesting magic link for {}", email);
        let result = service.request_magic_link(email).await;
        assert!(result.is_ok());
        let raw_token = result.unwrap();
        assert_eq!(raw_token.len(), 64); // 32 bytes hex encoded
        dbg!("  ✅  Result: Token generated successfully (len: {})", raw_token.len());

        // Verify it was stored in the repo
        let stored_link = repo.find_magic_link_by_email(email).await.unwrap();
        assert!(stored_link.is_some());
        let stored_link = stored_link.unwrap();
        
        // Verify hash matches
        assert_eq!(stored_link.token_hash, hash_token(&raw_token));
        assert_eq!(stored_link.email, email);
        assert!(!stored_link.used);
        dbg!("  ✅  Result: Token hash stored correctly in DB");

        // Test invalid email
        dbg!("  ➡️  Action: Requesting link for invalid email 'invalid-email'");
        let result = service.request_magic_link("invalid-email").await;
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
        dbg!("  ✅  Result: Rejected invalid email as expected");
    }

    #[tokio::test]
    async fn test_verify_magic_link() {
        dbg!("\n✨ TEST: test_verify_magic_link ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = MagicLinkService::new(repo.clone());
        let email = "user@example.com";

        // 1. Request a link (creates the record)
        dbg!("  ➡️  Action: Requesting initial magic link");
        let raw_token = service.request_magic_link(email).await.unwrap();

        // 2. Verify the link (Success case - New User)
        dbg!("  ➡️  Action: Verifying link for new user");
        let result = service.verify_magic_link(email, &raw_token, None, None).await;
        assert!(result.is_ok());
        
        let (user, session) = result.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(session.user_id, user.id);
        dbg!("  ✅  Result: New user created and session established");

        // Verify user was created in repo
        let repo_user = repo.find_user_by_email(email).await.unwrap();
        assert!(repo_user.is_some());
        assert_eq!(repo_user.unwrap().id, user.id);

        // Verify link is marked as used
        let stored_link = repo.find_magic_link_by_email(email).await.unwrap().unwrap();
        assert!(stored_link.used);
        dbg!("  ✅  Result: Link marked as used in DB");

        // 3. Try to use the same token again (Should fail)
        dbg!("  ➡️  Action: Attempting to reuse used token");
        let result = service.verify_magic_link(email, &raw_token, None, None).await;
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
        dbg!("  ✅  Result: Reuse rejected as expected");

        // 4. Test expired token
        dbg!("  ➡️  Action: Testing expired token scenario");
        let expired_token_str = "expired_token_value";
        let expired_hash = hash_token(expired_token_str);
        let expired_link = MagicLinkToken {
            email: "expired@example.com".to_string(),
            token_hash: expired_hash,
            expires_at: Utc::now() - Duration::minutes(1), // Expired
            used: false,
        };
        repo.save_magic_link(expired_link).await.unwrap();

        let result = service.verify_magic_link("expired@example.com", expired_token_str, None, None).await;
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
        dbg!("  ✅  Result: Expired token rejected");

        // 5. Test invalid token (wrong token for email)
        dbg!("  ➡️  Action: Testing wrong token scenario");
        let _ = service.request_magic_link("valid@example.com").await.unwrap();
        let result = service.verify_magic_link("valid@example.com", "wrong_token_value", None, None).await;
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
        dbg!("  ✅  Result: Wrong token rejected");

        // 6. Test Existing User Flow
        dbg!("  ➡️  Action: Testing existing user flow");
        let existing_user = User {
            id: Uuid::new_v4(),
            name: "Existing".to_string(),
            email: "existing@example.com".to_string(),
            phone: None,
            plan: UserPlan::Premium,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.create_user(existing_user.clone()).await.unwrap();

        let raw_token_existing = service.request_magic_link("existing@example.com").await.unwrap();
        
        let result = service.verify_magic_link("existing@example.com", &raw_token_existing, None, None).await;
        assert!(result.is_ok());
        let (user, _) = result.unwrap();
        
        assert_eq!(user.id, existing_user.id);
        assert!(user.last_sign_in.is_some());
        assert!(matches!(user.last_sign_in_method, Some(LastSignInMethod::MagicLink)));
        dbg!("  ✅  Result: Existing user logged in and updated");
    }

    #[test]
    fn test_email_validation() {
        dbg!("\n✨ TEST: test_email_validation ✨");
        assert!(is_valid_email("user@example.com"));
        assert!(!is_valid_email("invalid"));
        dbg!("  ✅  Result: Email validation logic correct");
    }

    #[test]
    fn test_token_generation() {
        dbg!("\n✨ TEST: test_token_generation ✨");
        let token = generate_random_token(32);
        assert_eq!(token.len(), 64);
        dbg!("  ✅  Result: Token length correct (64 chars)");
    }

    #[test]
    fn test_token_hashing() {
        dbg!("\n✨ TEST: test_token_hashing ✨");
        let token = "test_token_123";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64);
        dbg!("  ✅  Result: Hashing is deterministic and correct length");
    }
}