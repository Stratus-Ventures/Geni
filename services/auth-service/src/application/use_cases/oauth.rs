use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::application::app_error::{AppError, AppResult};
use crate::domain::models::*;
use crate::domain::ports::AuthRepository;

/// The OAuthService handles "Sign in with Google/Apple" authentication.
/// It manages linking external OAuth providers to user accounts.
pub struct OAuthService {
    repo: Arc<dyn AuthRepository>,
}

impl OAuthService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    /// Verify an OAuth login and return a session.
    ///
    /// **Flow:**
    /// 1. Check if a UserIdentity exists for this provider + provider_id.
    /// 2. If YES: The user has signed in with this OAuth account before -> Return Session.
    /// 3. If NO: Check if a User exists with this email.
    ///    - If User exists: Link the OAuth account to the existing user.
    ///    - If User doesn't exist: Create a new User + UserIdentity.
    /// 4. Return a Session.
    ///
    /// **Example:** A user signs up with magic link (email), then later clicks "Sign in with Google".
    /// Since the email matches, we link their Google account to their existing user record.
    pub async fn verify_oauth_login(
        &self,
        provider: OAuthProvider,
        provider_id: String,
        email: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<(User, Session)> {
        let normalized_email = email.to_lowercase();

        // Step 1: Check if this OAuth identity already exists
        if let Some(identity) = self
            .repo
            .find_user_identity(&provider, &provider_id)
            .await?
        {
            // This OAuth account is already linked to a user
            let user = self
                .repo
                .find_user_by_id(identity.user_id)
                .await?
                .ok_or_else(|| AppError::Internal("User not found for existing identity".to_string()))?;

            // Update last sign-in information
            let mut updated_user = user;
            updated_user.last_sign_in = Some(Utc::now());
            updated_user.last_sign_in_method = Some(LastSignInMethod::OAuth(provider.clone()));
            updated_user.updated_at = Utc::now();

            self.repo
                .update_user(updated_user.clone())
                .await?;

            // Create session
            let session = self.create_session(updated_user.id, ip_address, user_agent).await?;

            return Ok((updated_user, session));
        }

        // Step 2: OAuth identity doesn't exist - check if a user with this email exists
        let user = match self
            .repo
            .find_user_by_email(&normalized_email)
            .await?
        {
            Some(existing_user) => {
                // User exists - link this OAuth account to their existing account
                let identity = UserIdentity {
                    id: Uuid::new_v4(),
                    user_id: existing_user.id,
                    provider: provider.clone(),
                    provider_id: provider_id.clone(),
                    email: normalized_email.clone(),
                    created_at: Utc::now(),
                };

                self.repo
                    .create_user_identity(identity)
                    .await?;

                // Update last sign-in information
                let mut updated_user = existing_user;
                updated_user.last_sign_in = Some(Utc::now());
                updated_user.last_sign_in_method = Some(LastSignInMethod::OAuth(provider));
                updated_user.updated_at = Utc::now();

                self.repo
                    .update_user(updated_user.clone())
                    .await?;

                updated_user
            }
            None => {
                // No user exists - create a new user AND link the OAuth identity
                let new_user = User {
                    id: Uuid::new_v4(),
                    name: extract_name_from_email(&normalized_email),
                    email: normalized_email.clone(),
                    phone: None,
                    plan: UserPlan::FreeTrial,
                    last_sign_in: Some(Utc::now()),
                    last_sign_in_method: Some(LastSignInMethod::OAuth(provider.clone())),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                let created_user = self
                    .repo
                    .create_user(new_user)
                    .await?;

                // Create the OAuth identity link
                let identity = UserIdentity {
                    id: Uuid::new_v4(),
                    user_id: created_user.id,
                    provider,
                    provider_id,
                    email: normalized_email.clone(),
                    created_at: Utc::now(),
                };

                self.repo
                    .create_user_identity(identity)
                    .await?;

                created_user
            }
        };

        // Create session
        let session = self.create_session(user.id, ip_address, user_agent).await?;

        Ok((user, session))
    }

    /// Manually link a new OAuth provider to an existing user.
    ///
    /// **Use Case:** A user is already logged in (has a session) and wants to add
    /// "Sign in with Apple" as an additional authentication method.
    ///
    /// **Security:** This should only be called when the user is authenticated
    /// (the API layer should verify the session first).
    pub async fn link_account(
        &self,
        user_id: Uuid,
        provider: OAuthProvider,
        provider_id: String,
        email: String,
    ) -> AppResult<UserIdentity> {
        let normalized_email = email.to_lowercase();

        // Verify the user exists
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Check if this OAuth identity is already linked to ANY account
        if let Some(existing_identity) = self
            .repo
            .find_user_identity(&provider, &provider_id)
            .await?
        {
            // This OAuth account is already linked to another user
            if existing_identity.user_id != user.id {
                return Err(AppError::Internal(
                    "This OAuth account is already linked to another user".to_string(),
                ));
            }

            // Already linked to this user - return existing identity
            return Ok(existing_identity);
        }

        // Create the new OAuth identity link
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id: user.id,
            provider,
            provider_id,
            email: normalized_email,
            created_at: Utc::now(),
        };

        let created_identity = self
            .repo
            .create_user_identity(identity)
            .await?;

        Ok(created_identity)
    }

    // ==========================================
    // PRIVATE HELPER METHODS
    // ==========================================

    /// Create a new session for a user.
    async fn create_session(
        &self,
        user_id: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<Session> {
        let session = Session {
            id: generate_session_id(),
            user_id,
            expires_at: Utc::now() + Duration::days(30),
            ip_address,
            user_agent,
            created_at: Utc::now(),
        };

        self.repo
            .create_session(session)
            .await
            .map_err(AppError::from)
    }
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Extract a reasonable display name from an email address.
/// Example: "john.doe@example.com" -> "john.doe"
fn extract_name_from_email(email: &str) -> String {
    email
        .split('@')
        .next()
        .unwrap_or("User")
        .to_string()
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
        identities: Mutex<HashMap<(OAuthProvider, String), UserIdentity>>, // Key: (Provider, ProviderID)
        sessions: Mutex<HashMap<String, Session>>,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
                users_by_email: Mutex::new(HashMap::new()),
                identities: Mutex::new(HashMap::new()),
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
             users.remove(&id);
             Ok(())
        }

        // Identity methods
        async fn find_user_identity(&self, provider: &OAuthProvider, provider_id: &str) -> Result<Option<UserIdentity>, RepositoryError> {
            let identities = self.identities.lock().unwrap();
            Ok(identities.get(&(provider.clone(), provider_id.to_string())).cloned())
        }

        async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, RepositoryError> {
            let mut identities = self.identities.lock().unwrap();
            let key = (identity.provider.clone(), identity.provider_id.clone());
            if identities.contains_key(&key) {
                 return Err(RepositoryError::DuplicateKey("identity".to_string()));
            }
            identities.insert(key, identity.clone());
            Ok(identity)
        }
        
        async fn delete_user_identities_for_user(&self, _user_id: Uuid) -> Result<(), RepositoryError> { Ok(()) }

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
        
        async fn delete_all_sessions_for_user(&self, _user_id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        
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

        // Unused methods
        async fn save_magic_link(&self, _token: MagicLinkToken) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_magic_link_by_email(&self, _email: &str) -> Result<Option<MagicLinkToken>, RepositoryError> { Ok(None) }
        async fn mark_magic_link_as_used(&self, _email: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_magic_links_for_email(&self, _email: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> { Ok(()) }
        
        async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError> { Ok(passkey) }
        async fn find_passkey_by_credential_id(&self, _id: &str) -> Result<Option<Passkey>, RepositoryError> { Ok(None) }
        async fn find_passkeys_by_user_id(&self, _user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError> { Ok(vec![]) }
        async fn update_passkey_sign_count(&self, _credential_id: &str, _count: i64) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_passkeys_for_user(&self, _user_id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
    }

    #[tokio::test]
    async fn test_verify_oauth_login() {
        dbg!("\n✨ TEST: test_verify_oauth_login ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = OAuthService::new(repo.clone());

        // 1. Test New User Creation (No user, no identity)
        dbg!("  ➡️  Action: Login new user via Google");
        let email = "newuser@example.com".to_string();
        let provider_id = "google_123".to_string();
        let provider = OAuthProvider::Google;

        let result = service.verify_oauth_login(
            provider.clone(),
            provider_id.clone(),
            email.clone(),
            None,
            None
        ).await;

        assert!(result.is_ok());
        let (user, session) = result.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.name, "newuser"); // Extracted from email
        assert_eq!(session.user_id, user.id);
        dbg!("  ✅  Result: New user created and session returned");

        // Verify identity was created
        let identity = repo.find_user_identity(&provider, &provider_id).await.unwrap();
        assert!(identity.is_some());
        assert_eq!(identity.unwrap().user_id, user.id);

        // 2. Test Existing Identity (User logs in again)
        dbg!("  ➡️  Action: Login existing user via Google");
        let result_again = service.verify_oauth_login(
            provider.clone(),
            provider_id.clone(),
            email.clone(),
            None,
            None
        ).await;

        assert!(result_again.is_ok());
        let (user_again, _) = result_again.unwrap();
        assert_eq!(user_again.id, user.id);
        // Verify last_sign_in updated
        assert!(user_again.last_sign_in.unwrap() >= user.last_sign_in.unwrap());
        dbg!("  ✅  Result: Existing user found, sign-in time updated");

        // 3. Test Linking to Existing User (User exists by email, but no identity)
        dbg!("  ➡️  Action: Link Apple login to existing Google user (same email)");
        let existing_email = "existing@example.com".to_string();
        let existing_user = User {
            id: Uuid::new_v4(),
            name: "Existing".to_string(),
            email: existing_email.clone(),
            phone: None,
            plan: UserPlan::Premium,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.create_user(existing_user.clone()).await.unwrap();

        // Login with Apple with same email
        let apple_id = "apple_456".to_string();
        let apple_provider = OAuthProvider::Apple;

        let result_link = service.verify_oauth_login(
            apple_provider.clone(),
            apple_id.clone(),
            existing_email.clone(),
            None,
            None
        ).await;

        assert!(result_link.is_ok());
        let (linked_user, _) = result_link.unwrap();
        
        // Should be the same user
        assert_eq!(linked_user.id, existing_user.id);
        
        // Identity should be created
        let identity_link = repo.find_user_identity(&apple_provider, &apple_id).await.unwrap();
        assert!(identity_link.is_some());
        assert_eq!(identity_link.unwrap().user_id, existing_user.id);
        dbg!("  ✅  Result: Account linked successfully");
    }

    #[tokio::test]
    async fn test_link_account() {
        dbg!("\n✨ TEST: test_link_account ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = OAuthService::new(repo.clone());

        // Create a user
        let user = User {
            id: Uuid::new_v4(),
            name: "Linker".to_string(),
            email: "linker@example.com".to_string(),
            phone: None,
            plan: UserPlan::FreeTrial,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.create_user(user.clone()).await.unwrap();

        // 1. Link a new provider
        dbg!("  ➡️  Action: Manually linking Google account");
        let result = service.link_account(
            user.id,
            OAuthProvider::Google,
            "google_link_1".to_string(),
            user.email.clone()
        ).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity.user_id, user.id);
        assert!(matches!(identity.provider, OAuthProvider::Google));
        dbg!("  ✅  Result: Link successful");

        // 2. Try to link same provider again (Should return existing)
        dbg!("  ➡️  Action: Linking duplicate Google account");
        let result_again = service.link_account(
            user.id,
            OAuthProvider::Google,
            "google_link_1".to_string(),
            user.email.clone()
        ).await;
        assert!(result_again.is_ok());
        assert_eq!(result_again.unwrap().id, identity.id);
        dbg!("  ✅  Result: Duplicate handled gracefully (idempotent)");

        // 3. Try to link provider already linked to ANOTHER user
        dbg!("  ➡️  Action: Linking account already owned by another user");
        let other_user = User {
            id: Uuid::new_v4(),
            name: "Other".to_string(),
            email: "other@example.com".to_string(),
            phone: None,
            plan: UserPlan::FreeTrial,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.create_user(other_user.clone()).await.unwrap();

        // Create identity for other user manually
        let other_identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id: other_user.id,
            provider: OAuthProvider::Apple,
            provider_id: "apple_stolen".to_string(),
            email: other_user.email.clone(),
            created_at: Utc::now(),
        };
        repo.create_user_identity(other_identity).await.unwrap();

        // Try to link 'apple_stolen' to 'user'
        let result_fail = service.link_account(
            user.id,
            OAuthProvider::Apple,
            "apple_stolen".to_string(),
            user.email.clone()
        ).await;

        // Should fail because it's linked to someone else
        assert!(result_fail.is_err());
        dbg!("  ✅  Result: Link rejected as expected");
    }

    #[tokio::test]
    async fn test_create_session() {
        dbg!("\n✨ TEST: test_create_session ✨");
        // This test implicitly verifies the session creation logic via public methods,
        // ensuring the session has correct properties (expiry, user_id).
        let repo = Arc::new(MockAuthRepository::new());
        let service = OAuthService::new(repo.clone());

        let email = "session_test@example.com".to_string();
        let provider_id = "sess_1".to_string();
        
        dbg!("  ➡️  Action: Creating session via OAuth login");
        let (user, session) = service.verify_oauth_login(
            OAuthProvider::Google,
            provider_id,
            email,
            Some("127.0.0.1".to_string()),
            Some("Mozilla".to_string())
        ).await.unwrap();

        assert_eq!(session.user_id, user.id);
        assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));
        assert_eq!(session.user_agent, Some("Mozilla".to_string()));
        
        // Check expiry is roughly 30 days from now
        let days_diff = (session.expires_at - Utc::now()).num_days();
        assert!(days_diff >= 29 && days_diff <= 30);
        
        // Check session is persisted
        let saved_session = repo.find_session_by_id(&session.id).await.unwrap();
        assert!(saved_session.is_some());
        dbg!("  ✅  Result: Session properties verified and persisted");
    }
}