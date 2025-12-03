use base64::Engine;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::application::app_error::{AppError, AppResult};
use crate::domain::models::*;
use crate::domain::ports::{AuthRepository, ChallengeStore, WebAuthnVerifier};

/// The PasskeyService handles WebAuthn (Passkey) authentication.
/// This enables biometric authentication (FaceID, TouchID, Windows Hello, etc.).
pub struct PasskeyService {
    repo: Arc<dyn AuthRepository>,
    challenge_store: Arc<dyn ChallengeStore>,
    webauthn_verifier: Arc<dyn WebAuthnVerifier>,
}

impl PasskeyService {
    pub fn new(
        repo: Arc<dyn AuthRepository>,
        challenge_store: Arc<dyn ChallengeStore>,
        webauthn_verifier: Arc<dyn WebAuthnVerifier>,
    ) -> Self {
        Self {
            repo,
            challenge_store,
            webauthn_verifier,
        }
    }

    pub async fn start_registration(&self, user_id: Uuid) -> AppResult<RegistrationChallenge> {
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let (challenge, state) = self.webauthn_verifier
            .start_registration(user.id, &user.email, &user.name)
            .await?;

        let challenge_key = format!("reg_challenge:{}", user.id);
        self.challenge_store
            .store_challenge(&challenge_key, &state, 300)
            .await?;

        Ok(RegistrationChallenge {
            challenge,
            user_id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
        })
    }

    pub async fn finish_registration(
        &self,
        user_id: Uuid,
        response: RegistrationResponse,
    ) -> AppResult<Passkey> {
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let challenge_key = format!("reg_challenge:{}", user.id);
        let stored_challenge = self
            .challenge_store
            .get_challenge(&challenge_key)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        self.challenge_store
            .delete_challenge(&challenge_key)
            .await?;

        let response_json = serde_json::to_string(&response)
            .map_err(|e| AppError::Internal(format!("Failed to serialize response: {}", e)))?;

        let (credential_id, public_key) = self
            .webauthn_verifier
            .verify_registration(&stored_challenge, &response_json)
            .await?;

        let passkey = Passkey {
            id: Uuid::new_v4(),
            user_id: user.id,
            credential_id: credential_id.clone(),
            public_key,
            sign_count: 0,
            transports: response.transports,
            device_name: response.device_name,
            last_used_at: Utc::now(),
            created_at: Utc::now(),
        };

        let saved_passkey = self
            .repo
            .save_passkey(passkey)
            .await?;

        Ok(saved_passkey)
    }

    pub async fn start_authentication(&self, email: &str) -> AppResult<AuthenticationChallenge> {
        let normalized_email = email.to_lowercase();

        let user = self
            .repo
            .find_user_by_email(&normalized_email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let passkeys = self
            .repo
            .find_passkeys_by_user_id(user.id)
            .await?;

        if passkeys.is_empty() {
            return Err(AppError::InvalidCredentials);
        }

        let (challenge, state) = self.webauthn_verifier
            .start_authentication(&passkeys)
            .await?;

        let challenge_key = format!("auth_challenge:{}", normalized_email);
        self.challenge_store
            .store_challenge(&challenge_key, &state, 300)
            .await?;

        Ok(AuthenticationChallenge {
            challenge,
            credential_ids: passkeys.iter().map(|pk| pk.credential_id.clone()).collect(),
        })
    }

    pub async fn finish_authentication(
        &self,
        response: AuthenticationResponse,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<(User, Session)> {
        let passkey = self
            .repo
            .find_passkey_by_credential_id(&response.credential_id)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let user = self
            .repo
            .find_user_by_id(passkey.user_id)
            .await?
            .ok_or_else(|| AppError::Internal("User not found for passkey".to_string()))?;

        let challenge_key = format!("auth_challenge:{}", user.email.to_lowercase());
        let stored_challenge = self
            .challenge_store
            .get_challenge(&challenge_key)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        self.challenge_store
            .delete_challenge(&challenge_key)
            .await?;

        let response_json = serde_json::to_string(&response)
            .map_err(|e| AppError::Internal(format!("Failed to serialize response: {}", e)))?;

        let verified_sign_count = self
            .webauthn_verifier
            .verify_authentication(
                &stored_challenge,
                &response_json,
            )
            .await?;

        if verified_sign_count <= passkey.sign_count {
            return Err(AppError::Internal(
                "Invalid sign_count: possible replay attack detected".to_string(),
            ));
        }

        self.repo
            .update_passkey_sign_count(&passkey.credential_id, verified_sign_count)
            .await?;

        let mut updated_user = user;
        updated_user.last_sign_in = Some(Utc::now());
        updated_user.last_sign_in_method = Some(LastSignInMethod::Passkey);
        updated_user.updated_at = Utc::now();

        self.repo
            .update_user(updated_user.clone())
            .await?;

        let session = Session {
            id: generate_session_id(),
            user_id: updated_user.id,
            expires_at: Utc::now() + Duration::days(30),
            ip_address,
            user_agent,
            created_at: Utc::now(),
        };

        let created_session = self
            .repo
            .create_session(session)
            .await?;

        Ok((updated_user, created_session))
    }
}

// ==========================================
// REQUEST/RESPONSE TYPES
// ==========================================

#[derive(Debug, Serialize)]
pub struct RegistrationChallenge {
    pub challenge: String,
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrationResponse {
    pub credential_id: String,
    pub client_data_json: String,
    pub attestation_object: String,
    pub transports: Option<Vec<String>>,
    pub device_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthenticationChallenge {
    pub challenge: String,
    pub credential_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
    pub credential_id: String,
    pub authenticator_data: String,
    pub client_data_json: String,
    pub signature: String,
    pub sign_count: i64,
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.r#gen()).collect();
    hex::encode(bytes)
}

pub fn base64_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn base64_decode(data: &str) -> AppResult<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .map_err(|e| AppError::Internal(format!("Invalid base64: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use async_trait::async_trait;
    use crate::domain::errors::{RepositoryError, ChallengeStoreError, WebAuthnError};

    // ==========================================
    // MOCKS
    // ==========================================

    // Mock Repository (Simplified for Passkeys)
    struct MockAuthRepository {
        users: Mutex<HashMap<Uuid, User>>,
        users_by_email: Mutex<HashMap<String, Uuid>>,
        passkeys: Mutex<HashMap<String, Passkey>>, // Keyed by credential_id
        sessions: Mutex<HashMap<String, Session>>,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
                users_by_email: Mutex::new(HashMap::new()),
                passkeys: Mutex::new(HashMap::new()),
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

        async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError> {
            let mut passkeys = self.passkeys.lock().unwrap();
            passkeys.insert(passkey.credential_id.clone(), passkey.clone());
            Ok(passkey)
        }

        async fn find_passkey_by_credential_id(&self, credential_id: &str) -> Result<Option<Passkey>, RepositoryError> {
            let passkeys = self.passkeys.lock().unwrap();
            Ok(passkeys.get(credential_id).cloned())
        }

        async fn find_passkeys_by_user_id(&self, user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError> {
            let passkeys = self.passkeys.lock().unwrap();
            Ok(passkeys.values()
                .filter(|pk| pk.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn update_passkey_sign_count(&self, credential_id: &str, count: i64) -> Result<(), RepositoryError> {
            let mut passkeys = self.passkeys.lock().unwrap();
            if let Some(pk) = passkeys.get_mut(credential_id) {
                pk.sign_count = count;
                Ok(())
            } else {
                Err(RepositoryError::NotFound("Passkey".to_string()))
            }
        }

        async fn create_session(&self, session: Session) -> Result<Session, RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        // Implement other methods required by trait (mostly stubs)
        async fn delete_user(&self, _id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_user_identity(&self, _p: &OAuthProvider, _pid: &str) -> Result<Option<UserIdentity>, RepositoryError> { Ok(None) }
        async fn create_user_identity(&self, _i: UserIdentity) -> Result<UserIdentity, RepositoryError> { Err(RepositoryError::Internal("Not impl".to_string())) }
        async fn delete_user_identities_for_user(&self, _id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_passkeys_for_user(&self, _id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn save_magic_link(&self, _t: MagicLinkToken) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_magic_link_by_email(&self, _e: &str) -> Result<Option<MagicLinkToken>, RepositoryError> { Ok(None) }
        async fn mark_magic_link_as_used(&self, _e: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_magic_links_for_email(&self, _e: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> { Ok(()) }
        async fn find_session_by_id(&self, _id: &str) -> Result<Option<Session>, RepositoryError> { Ok(None) }
        async fn update_session(&self, _s: Session) -> Result<Session, RepositoryError> { Err(RepositoryError::Internal("Not impl".to_string())) }
        async fn delete_session(&self, _id: &str) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_all_sessions_for_user(&self, _id: Uuid) -> Result<(), RepositoryError> { Ok(()) }
        async fn delete_expired_sessions(&self) -> Result<(), RepositoryError> { Ok(()) }
    }

    // Mock Challenge Store
    struct MockChallengeStore {
        challenges: Mutex<HashMap<String, String>>,
    }

    impl MockChallengeStore {
        fn new() -> Self {
            Self { challenges: Mutex::new(HashMap::new()) }
        }
    }

    #[async_trait]
    impl ChallengeStore for MockChallengeStore {
        async fn store_challenge(&self, key: &str, challenge: &str, _ttl: u64) -> Result<(), ChallengeStoreError> {
            let mut store = self.challenges.lock().unwrap();
            store.insert(key.to_string(), challenge.to_string());
            Ok(())
        }

        async fn get_challenge(&self, key: &str) -> Result<Option<String>, ChallengeStoreError> {
            let store = self.challenges.lock().unwrap();
            Ok(store.get(key).cloned())
        }

        async fn delete_challenge(&self, key: &str) -> Result<(), ChallengeStoreError> {
            let mut store = self.challenges.lock().unwrap();
            store.remove(key);
            Ok(())
        }
    }

    // Mock WebAuthn Verifier
    struct MockWebAuthnVerifier;

    impl MockWebAuthnVerifier {
        fn new() -> Self { Self }
    }

    #[async_trait]
    impl WebAuthnVerifier for MockWebAuthnVerifier {
        async fn start_registration(
            &self,
            _user_id: Uuid,
            _email: &str,
            _name: &str,
        ) -> Result<(String, String), WebAuthnError> {
            Ok(("{\"challenge\":\"mock_challenge\"}".to_string(), "{\"state\":\"mock_reg_state\"}".to_string()))
        }

        async fn verify_registration(
            &self,
            _state_json: &str,
            _response_json: &str,
        ) -> Result<(String, Vec<u8>), WebAuthnError> {
            Ok(("cred_123".to_string(), vec![1, 2, 3, 4]))
        }

        async fn start_authentication(
            &self,
            _allow_credentials: &[Passkey],
        ) -> Result<(String, String), WebAuthnError> {
            Ok(("{\"challenge\":\"mock_auth_challenge\"}".to_string(), "{\"state\":\"mock_auth_state\"}".to_string()))
        }

        async fn verify_authentication(
            &self,
            _state_json: &str,
            _response_json: &str,
        ) -> Result<i64, WebAuthnError> {
            Ok(10)
        }
    }

    // Helper
    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "passkey@example.com".to_string(),
            phone: None,
            plan: UserPlan::FreeTrial,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_start_registration() {
        println!("\n✨ TEST: test_start_registration ✨");
        println!("  ➡️  Action: Initializing Mocks and Service");
        let repo = Arc::new(MockAuthRepository::new());
        let store = Arc::new(MockChallengeStore::new());
        let verifier = Arc::new(MockWebAuthnVerifier::new());
        let service = PasskeyService::new(repo.clone(), store.clone(), verifier.clone());

        let user = create_test_user();
        repo.create_user(user.clone()).await.unwrap();

        println!("  ➡️  Action: Starting registration for user {}", user.email);
        let result = service.start_registration(user.id).await;
        assert!(result.is_ok());
        
        let challenge_resp = result.unwrap();
        println!("  ✅  Result: Challenge generated: {}", challenge_resp.challenge);
        
        // Verify challenge stored
        let key = format!("reg_challenge:{}", user.id);
        let stored = store.get_challenge(&key).await.unwrap();
        assert!(stored.is_some());
        // The challenge in response is now the JSON string, and stored is the state string
        // Mock returns "mock_challenge" JSON and "mock_reg_state" state
        assert!(challenge_resp.challenge.contains("mock_challenge"));
        assert!(stored.unwrap().contains("mock_reg_state"));
    }

    #[tokio::test]
    async fn test_finish_registration() {
        println!("\n✨ TEST: test_finish_registration ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let store = Arc::new(MockChallengeStore::new());
        let verifier = Arc::new(MockWebAuthnVerifier::new());
        let service = PasskeyService::new(repo.clone(), store.clone(), verifier.clone());

        let user = create_test_user();
        repo.create_user(user.clone()).await.unwrap();

        // Setup challenge state
        let state_json = "{\"state\":\"mock_reg_state\"}";
        let key = format!("reg_challenge:{}", user.id);
        store.store_challenge(&key, state_json, 300).await.unwrap();

        // Fake response
        let response = RegistrationResponse {
            credential_id: "cred_123".to_string(),
            client_data_json: base64_encode(b"{{}}"),
            attestation_object: base64_encode(b"{{}}"),
            transports: None,
            device_name: Some("Test Device".to_string()),
        };

        println!("  ➡️  Action: Finishing registration");
        let result = service.finish_registration(user.id, response).await;
        assert!(result.is_ok());
        let passkey = result.unwrap();
        
        println!("  ✅  Result: Passkey created with ID: {}", passkey.credential_id);
        assert_eq!(passkey.credential_id, "cred_123");
        assert_eq!(passkey.user_id, user.id);

        // Verify challenge deleted
        assert!(store.get_challenge(&key).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_start_authentication() {
        println!("\n✨ TEST: test_start_authentication ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let store = Arc::new(MockChallengeStore::new());
        let verifier = Arc::new(MockWebAuthnVerifier::new());
        let service = PasskeyService::new(repo.clone(), store.clone(), verifier.clone());

        let user = create_test_user();
        repo.create_user(user.clone()).await.unwrap();

        // Must have a passkey first
        let passkey = Passkey {
            id: Uuid::new_v4(),
            user_id: user.id,
            credential_id: "cred_auth_test".to_string(),
            public_key: vec![],
            sign_count: 0,
            transports: None,
            device_name: None,
            last_used_at: Utc::now(),
            created_at: Utc::now(),
        };
        repo.save_passkey(passkey).await.unwrap();

        println!("  ➡️  Action: Starting auth for {}", user.email);
        let result = service.start_authentication(&user.email).await;
        assert!(result.is_ok());
        let auth_challenge = result.unwrap();

        println!("  ✅  Result: Auth challenge generated: {}", auth_challenge.challenge);
        // The challenge field now contains the JSON from the adapter mock
        assert!(auth_challenge.challenge.contains("mock_auth_challenge"));
        
        // Check credential IDs are still populated for hints
        assert!(auth_challenge.credential_ids.contains(&"cred_auth_test".to_string()));

        // Verify challenge state stored
        let key = format!("auth_challenge:{}", user.email);
        let stored_state = store.get_challenge(&key).await.unwrap();
        assert!(stored_state.is_some());
        assert!(stored_state.unwrap().contains("mock_auth_state"));
    }

    #[tokio::test]
    async fn test_finish_authentication() {
        println!("\n✨ TEST: test_finish_authentication ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let store = Arc::new(MockChallengeStore::new());
        let verifier = Arc::new(MockWebAuthnVerifier::new());
        let service = PasskeyService::new(repo.clone(), store.clone(), verifier.clone());

        let user = create_test_user();
        repo.create_user(user.clone()).await.unwrap();

        // Save Passkey with sign_count 5
        let passkey = Passkey {
            id: Uuid::new_v4(),
            user_id: user.id,
            credential_id: "cred_finish_auth".to_string(),
            public_key: vec![],
            sign_count: 5,
            transports: None,
            device_name: None,
            last_used_at: Utc::now(),
            created_at: Utc::now(),
        };
        repo.save_passkey(passkey).await.unwrap();

        // Setup challenge state
        let key = format!("auth_challenge:{}", user.email);
        store.store_challenge(&key, "{\"state\":\"mock_auth_state\"}", 300).await.unwrap();

        // Response (MockVerifier returns sign_count 10)
        let response = AuthenticationResponse {
            credential_id: "cred_finish_auth".to_string(),
            authenticator_data: base64_encode(b"data"),
            client_data_json: base64_encode(b"client"),
            signature: base64_encode(b"sig"),
            sign_count: 999, // Client provided, ignored by logic which uses verifier result (10)
        };

        println!("  ➡️  Action: Finishing authentication");
        let result = service.finish_authentication(response, None, None).await;
        assert!(result.is_ok());
        let (updated_user, session) = result.unwrap();

        println!("  ✅  Result: Session created: {}", session.id);
        assert_eq!(updated_user.id, user.id);
        assert!(matches!(updated_user.last_sign_in_method, Some(LastSignInMethod::Passkey)));

        // Check sign count updated in DB (MockVerifier returns 10)
        let stored_pk = repo.find_passkey_by_credential_id("cred_finish_auth").await.unwrap().unwrap();
        assert_eq!(stored_pk.sign_count, 10);
    }

    #[test]
    fn test_generate_session_id() {
        println!("\n✨ TEST: test_generate_session_id ✨");
        let id1 = generate_session_id();
        assert_eq!(id1.len(), 64);
        println!("  ✅  Result: Session ID len is 64 hex chars");
    }

    #[test]
    fn test_base64_encoding() {
        println!("\n✨ TEST: test_base64_encoding ✨");
        let data = vec![1, 2, 3, 4, 5];
        let encoded = base64_encode(&data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(data, decoded);
        println!("  ✅  Result: Encode/Decode cycle successful");
    }
}