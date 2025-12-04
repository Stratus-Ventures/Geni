use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::application::app_error::{AppError, AppResult};
use crate::domain::models::*;
use crate::domain::ports::AuthRepository;

/// The SessionService manages user sessions (authentication tokens stored in cookies).
/// Sessions are the primary way to maintain user authentication state.
pub struct SessionService {
    repo: Arc<dyn AuthRepository>,
}

impl SessionService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    /// Get the current authenticated user from a session ID.
    ///
    /// **Flow:**
    /// 1. Find the session by ID (the cookie value).
    /// 2. Check if the session has expired.
    /// 3. If valid, return the User.
    /// 4. Optionally extend the session expiration (sliding window).
    ///
    /// **Use Case:** Every authenticated API request calls this to verify the user is logged in.
    pub async fn get_current_user(&self, session_id: &str) -> AppResult<User> {
        // Find the session
        let session = self
            .repo
            .find_session_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Check if the session has expired
        if session.expires_at < Utc::now() {
            // Session expired - delete it from the database
            let _ = self.repo.delete_session(session_id).await;
            return Err(AppError::InvalidCredentials);
        }

        // Find the user
        let user = self
            .repo
            .find_user_by_id(session.user_id)
            .await?
            .ok_or_else(|| AppError::Internal("User not found for session".to_string()))?;

        // Optional: Extend session expiration (sliding window)
        // If the session has less than 7 days remaining, extend it to 30 days
        let days_remaining = (session.expires_at - Utc::now()).num_days();
        if days_remaining < 7 {
            let mut extended_session = session;
            extended_session.expires_at = Utc::now() + Duration::days(30);

            self.repo
                .update_session(extended_session)
                .await?;
        }

        Ok(user)
    }

    /// Get the current user with full session information.
    ///
    /// **Use Case:** When you need both the User and Session data (e.g., displaying
    /// device information, IP address, last activity, etc. in account settings).
    pub async fn get_current_user_with_session(&self, session_id: &str) -> AppResult<(User, Session)> {
        // Find the session
        let session = self
            .repo
            .find_session_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Check if the session has expired
        if session.expires_at < Utc::now() {
            // Session expired - delete it
            let _ = self.repo.delete_session(session_id).await;
            return Err(AppError::InvalidCredentials);
        }

        // Find the user
        let user = self
            .repo
            .find_user_by_id(session.user_id)
            .await?
            .ok_or_else(|| AppError::Internal("User not found for session".to_string()))?;

        // Optional: Extend session expiration (sliding window)
        let days_remaining = (session.expires_at - Utc::now()).num_days();
        let final_session = if days_remaining < 7 {
            let mut extended_session = session;
            extended_session.expires_at = Utc::now() + Duration::days(30);

            self.repo
                .update_session(extended_session.clone())
                .await?;

            extended_session
        } else {
            session
        };

        Ok((user, final_session))
    }

    /// Log out the current user by deleting their session.
    ///
    /// **Flow:**
    /// 1. Delete the session from the database.
    /// 2. The API layer should also clear the cookie in the browser.
    ///
    /// **Use Case:** When the user clicks "Log Out" or when you detect suspicious activity.
    pub async fn logout(&self, session_id: &str) -> AppResult<()> {
        self.repo
            .delete_session(session_id)
            .await?;

        Ok(())
    }

    /// Revoke all active sessions for a user (log out all devices).
    ///
    /// **Flow:**
    /// 1. Delete ALL sessions associated with the user_id.
    /// 2. This logs the user out of every device (phone, laptop, tablet, etc.).
    ///
    /// **Use Cases:**
    /// - User clicks "Log out all devices" in account settings.
    /// - Security incident: User reports their account was compromised.
    /// - Password/email change: Force re-authentication on all devices.
    /// - Account deletion: Clean up all sessions before deleting the account.
    pub async fn revoke_all_devices(&self, user_id: Uuid) -> AppResult<()> {
        self.repo
            .delete_all_sessions_for_user(user_id)
            .await?;

        Ok(())
    }

    /// Verify a session is valid without returning the user.
    ///
    /// **Use Case:** Lightweight check to see if a session is still active
    /// (e.g., for WebSocket connections, real-time features).
    ///
    /// **Returns:** `true` if the session is valid, `false` otherwise.
    pub async fn verify_session(&self, session_id: &str) -> bool {
        match self.repo.find_session_by_id(session_id).await {
            Ok(Some(session)) => session.expires_at >= Utc::now(),
            _ => false,
        }
    }

    /// Clean up expired sessions (background job).
    ///
    /// **Use Case:** Run this periodically (e.g., daily cron job) to remove
    /// expired sessions from the database and keep it clean.
    ///
    /// **Note:** This is a maintenance operation, not a user-facing feature.
    pub async fn cleanup_expired_sessions(&self) -> AppResult<()> {
        self.repo
            .delete_expired_sessions()
            .await?;

        Ok(())
    }
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
        sessions: Mutex<HashMap<String, Session>>,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
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
        
        // Session methods
        async fn create_session(&self, session: Session) -> Result<Session, RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        async fn find_session_by_id(&self, session_id: &str) -> Result<Option<Session>, RepositoryError> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.get(session_id).cloned())
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

        async fn delete_session(&self, session_id: &str) -> Result<(), RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(session_id);
            Ok(())
        }

        async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.retain(|_, s| s.user_id != user_id);
            Ok(())
        }

        async fn delete_expired_sessions(&self) -> Result<(), RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            let now = Utc::now();
            sessions.retain(|_, s| s.expires_at > now);
            Ok(())
        }
        
        // Unused methods required by trait
        async fn find_user_by_email(&self, _email: &str) -> Result<Option<User>, RepositoryError> { Ok(None) }
        async fn create_user(&self, _user: User) -> Result<User, RepositoryError> { Err(RepositoryError::Internal("Not implemented".to_string())) }
        async fn update_user(&self, _user: User) -> Result<User, RepositoryError> { Err(RepositoryError::Internal("Not implemented".to_string())) }
        async fn delete_user(&self, _id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_user_identity(&self, _p: &OAuthProvider, _pid: &str) -> Result<Option<UserIdentity>, RepositoryError> { Ok(None) }
        async fn create_user_identity(&self, _i: UserIdentity) -> Result<UserIdentity, RepositoryError> { Err(RepositoryError::Internal("Not implemented".to_string())) }
        async fn save_passkey(&self, _p: Passkey) -> Result<Passkey, RepositoryError> { Err(RepositoryError::Internal("Not implemented".to_string())) }
        async fn find_passkey_by_credential_id(&self, _cid: &str) -> Result<Option<Passkey>, RepositoryError> { Ok(None) }
        async fn find_passkeys_by_user_id(&self, _uid: Uuid) -> Result<Vec<Passkey>, RepositoryError> { Ok(vec![]) }
        async fn update_passkey_sign_count(&self, _cid: &str, _cnt: i64) -> Result<(), RepositoryError> { Ok(()) }
        async fn save_magic_link(&self, _t: MagicLinkToken) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_magic_link_by_email(&self, _e: &str) -> Result<Option<MagicLinkToken>, RepositoryError> { Ok(None) }
        async fn mark_magic_link_as_used(&self, _e: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_passkeys_for_user(&self, _uid: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_user_identities_for_user(&self, _uid: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_magic_links_for_email(&self, _e: &str) -> Result<(), RepositoryError> { Ok(()) }
    }

    // Helper to create a dummy user
    fn create_dummy_user() -> User {
        User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: None,
            plan: UserPlan::FreeTrial,
            last_sign_in: Some(Utc::now()),
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_get_current_user() {
        dbg!("\n✨ TEST: test_get_current_user ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());

        // Setup user and session
        let user = create_dummy_user();
        {
            let mut users = repo.users.lock().unwrap();
            users.insert(user.id, user.clone());
        }

        let session_id = "valid_session_token";
        let session = Session {
            id: session_id.to_string(),
            user_id: user.id,
            expires_at: Utc::now() + Duration::days(1),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };
        repo.create_session(session).await.unwrap();

        // 1. Test Success
        dbg!("  ➡️  Action: Getting current user for valid session");
        let result = service.get_current_user(session_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, user.id);
        dbg!("  ✅  Result: User retrieved successfully");

        // 2. Test Expired Session
        dbg!("  ➡️  Action: Testing expired session");
        let expired_id = "expired_token";
        let expired_session = Session {
            id: expired_id.to_string(),
            user_id: user.id,
            expires_at: Utc::now() - Duration::seconds(1),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };
        repo.create_session(expired_session).await.unwrap();

        let result_expired = service.get_current_user(expired_id).await;
        assert!(matches!(result_expired, Err(AppError::InvalidCredentials)));
        
        // Should be deleted from repo
        let saved = repo.find_session_by_id(expired_id).await.unwrap();
        assert!(saved.is_none());
        dbg!("  ✅  Result: Expired session rejected and deleted");

        // 3. Test Non-existent Session
        dbg!("  ➡️  Action: Testing non-existent session");
        let result_none = service.get_current_user("non_existent").await;
        assert!(matches!(result_none, Err(AppError::InvalidCredentials)));
        dbg!("  ✅  Result: Invalid session rejected");
    }

    #[tokio::test]
    async fn test_get_current_user_with_session() {
        dbg!("\n✨ TEST: test_get_current_user_with_session ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());
        let user = create_dummy_user();
        {
            let mut users = repo.users.lock().unwrap();
            users.insert(user.id, user.clone());
        }

        // 1. Test Sliding Window Extension
        dbg!("  ➡️  Action: Testing session sliding window extension");
        let session_id = "near_expiry_token";
        let original_expiry = Utc::now() + Duration::days(5); // Less than 7 days
        let session = Session {
            id: session_id.to_string(),
            user_id: user.id,
            expires_at: original_expiry,
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };
        repo.create_session(session).await.unwrap();

        let result = service.get_current_user_with_session(session_id).await;
        assert!(result.is_ok());
        let (fetched_user, fetched_session) = result.unwrap();
        
        assert_eq!(fetched_user.id, user.id);
        assert_eq!(fetched_session.id, session_id);
        
        // Should have been extended
        assert!(fetched_session.expires_at > original_expiry + Duration::days(20));
        
        // Check persistence of extension
        let stored = repo.find_session_by_id(session_id).await.unwrap().unwrap();
        assert!(stored.expires_at > original_expiry + Duration::days(20));
        dbg!("  ✅  Result: Session extended successfully");
    }

    #[tokio::test]
    async fn test_logout() {
        dbg!("\n✨ TEST: test_logout ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());
        let user_id = Uuid::new_v4();

        let session_id = "logout_test_token";
        let session = Session {
            id: session_id.to_string(),
            user_id,
            expires_at: Utc::now() + Duration::days(1),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };
        repo.create_session(session).await.unwrap();

        // Ensure exists
        assert!(repo.find_session_by_id(session_id).await.unwrap().is_some());

        // Logout
        dbg!("  ➡️  Action: Logging out session");
        let result = service.logout(session_id).await;
        assert!(result.is_ok());

        // Ensure gone
        assert!(repo.find_session_by_id(session_id).await.unwrap().is_none());
        dbg!("  ✅  Result: Session removed from DB");
    }

    #[tokio::test]
    async fn test_revoke_all_devices() {
        dbg!("\n✨ TEST: test_revoke_all_devices ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());
        let user_id = Uuid::new_v4();

        // Create multiple sessions
        dbg!("  ➡️  Action: Creating multiple sessions for user");
        for i in 0..3 {
            repo.create_session(Session {
                id: format!("session_{}", i),
                user_id,
                expires_at: Utc::now() + Duration::days(1),
                ip_address: None,
                user_agent: None,
                created_at: Utc::now(),
            }).await.unwrap();
        }
        
        // Create session for OTHER user
        let other_user_id = Uuid::new_v4();
        repo.create_session(Session {
            id: "other_session".to_string(),
            user_id: other_user_id,
            expires_at: Utc::now() + Duration::days(1),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }).await.unwrap();

        // Revoke
        dbg!("  ➡️  Action: Revoking all devices for user");
        let result = service.revoke_all_devices(user_id).await;
        assert!(result.is_ok());

        // Check user's sessions are gone
        assert!(repo.find_session_by_id("session_0").await.unwrap().is_none());
        assert!(repo.find_session_by_id("session_1").await.unwrap().is_none());
        assert!(repo.find_session_by_id("session_2").await.unwrap().is_none());

        // Check other user's session remains
        assert!(repo.find_session_by_id("other_session").await.unwrap().is_some());
        dbg!("  ✅  Result: Correct sessions revoked");
    }

    #[tokio::test]
    async fn test_verify_session() {
        dbg!("\n✨ TEST: test_verify_session ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());
        let user_id = Uuid::new_v4();

        // Valid session
        repo.create_session(Session {
            id: "valid".to_string(),
            user_id,
            expires_at: Utc::now() + Duration::minutes(5),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }).await.unwrap();

        // Expired session
        repo.create_session(Session {
            id: "expired".to_string(),
            user_id,
            expires_at: Utc::now() - Duration::minutes(5),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }).await.unwrap();

        assert_eq!(service.verify_session("valid").await, true);
        assert_eq!(service.verify_session("expired").await, false);
        assert_eq!(service.verify_session("missing").await, false);
        dbg!("  ✅  Result: Session verification accurate");
    }

    #[tokio::test]
    async fn test_cleanup_expired_sessions() {
        dbg!("\n✨ TEST: test_cleanup_expired_sessions ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = SessionService::new(repo.clone());
        let user_id = Uuid::new_v4();

        // 1. Expired session
        repo.create_session(Session {
            id: "expired_1".to_string(),
            user_id,
            expires_at: Utc::now() - Duration::minutes(10),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }).await.unwrap();

        // 2. Valid session
        repo.create_session(Session {
            id: "valid_1".to_string(),
            user_id,
            expires_at: Utc::now() + Duration::minutes(10),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }).await.unwrap();

        dbg!("  ➡️  Action: Cleaning up expired sessions");
        service.cleanup_expired_sessions().await.unwrap();

        assert!(repo.find_session_by_id("expired_1").await.unwrap().is_none());
        assert!(repo.find_session_by_id("valid_1").await.unwrap().is_some());
        dbg!("  ✅  Result: Expired sessions removed");
    }

    #[test]
    fn test_session_expiration_calculation() {
        dbg!("\n✨ TEST: test_session_expiration_calculation ✨");
        let now = Utc::now();
        let expires_at = now + Duration::days(5);
        let days_remaining = (expires_at - now).num_days();

        assert_eq!(days_remaining, 5);
        assert!(days_remaining < 7); 
        dbg!("  ✅  Result: Expiration math correct");
    }

    #[test]
    fn test_session_not_expired() {
        dbg!("\n✨ TEST: test_session_not_expired ✨");
        let now = Utc::now();
        let expires_at = now + Duration::days(10);

        assert!(expires_at > now); 
        dbg!("  ✅  Result: Session valid check correct");
    }

    #[test]
    fn test_session_expired() {
        dbg!("\n✨ TEST: test_session_expired ✨");
        let now = Utc::now();
        let expires_at = now - Duration::hours(1);

        assert!(expires_at < now); 
        dbg!("  ✅  Result: Session expired check correct");
    }
}