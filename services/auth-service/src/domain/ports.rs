use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::models::*;
use crate::domain::errors::{RepositoryError, ChallengeStoreError, WebAuthnError};

/// The AuthRepository trait defines ALL data access operations needed by the Application layer.
/// This is the "Port" in Hexagonal Architecture (aka Clean Architecture).
/// The Infrastructure layer will implement this trait with concrete database adapters (SQLx, etc.).
#[async_trait]
pub trait AuthRepository: Send + Sync {
    // ==========================================
    // 1. USER MANAGEMENT
    // ==========================================

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn create_user(&self, user: User) -> Result<User, RepositoryError>;
    async fn update_user(&self, user: User) -> Result<User, RepositoryError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;

    // ==========================================
    // 2. OAUTH / IDENTITY MANAGEMENT
    // ==========================================

    async fn find_user_identity(&self, provider: &OAuthProvider, provider_id: &str) -> Result<Option<UserIdentity>, RepositoryError>;
    async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, RepositoryError>;
    async fn delete_user_identities_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;

    // ==========================================
    // 3. PASSKEYS (WEBAUTHN)
    // ==========================================

    async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError>;
    async fn find_passkey_by_credential_id(&self, credential_id: &str) -> Result<Option<Passkey>, RepositoryError>;
    async fn find_passkeys_by_user_id(&self, user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError>;
    async fn update_passkey_sign_count(&self, credential_id: &str, new_sign_count: i64) -> Result<(), RepositoryError>;
    async fn delete_passkeys_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;

    // ==========================================
    // 4. MAGIC LINKS
    // ==========================================

    async fn save_magic_link(&self, magic_link: MagicLinkToken) -> Result<(), RepositoryError>;
    async fn find_magic_link_by_email(&self, email: &str) -> Result<Option<MagicLinkToken>, RepositoryError>;
    async fn mark_magic_link_as_used(&self, email: &str) -> Result<(), RepositoryError>;
    async fn delete_magic_links_for_email(&self, email: &str) -> Result<(), RepositoryError>;
    async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError>;

    // ==========================================
    // 5. SESSIONS
    // ==========================================

    async fn create_session(&self, session: Session) -> Result<Session, RepositoryError>;
    async fn find_session_by_id(&self, session_id: &str) -> Result<Option<Session>, RepositoryError>;
    async fn update_session(&self, session: Session) -> Result<Session, RepositoryError>;
    async fn delete_session(&self, session_id: &str) -> Result<(), RepositoryError>;
    async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_expired_sessions(&self) -> Result<(), RepositoryError>;
}

/// The ChallengeStore trait defines temporary storage for WebAuthn challenges.
/// **Purpose:**
/// Prevents replay attacks by ensuring the challenge used in the WebAuthn ceremony
/// matches the one we generated and sent to the client.

#[async_trait]
pub trait ChallengeStore: Send + Sync {
    async fn store_challenge(&self, key: &str, challenge: &str, ttl_seconds: u64) -> Result<(), ChallengeStoreError>;
    async fn get_challenge(&self, key: &str) -> Result<Option<String>, ChallengeStoreError>;
    async fn delete_challenge(&self, key: &str) -> Result<(), ChallengeStoreError>;
}

/// The WebAuthnVerifier trait defines cryptographic verification for WebAuthn operations.
/// This abstracts the complex WebAuthn verification logic.
///
/// **Refactored for webauthn-rs compatibility:**
/// - `start_*` methods return (ClientJSON, StateJSON).
/// - `verify_*` methods take (StateJSON, ResponseJSON) and return the result (PublicKey or SignCount).
#[async_trait]
pub trait WebAuthnVerifier: Send + Sync {
    async fn start_registration(
        &self,
        user_id: Uuid,
        email: &str,
        name: &str,
    ) -> Result<(String, String), WebAuthnError>;

    async fn verify_registration(
        &self,
        state_json: &str,
        response_json: &str,
    ) -> Result<(String, Vec<u8>), WebAuthnError>; // Returns (credential_id, public_key)

    async fn start_authentication(
        &self,
        allow_credentials: &[Passkey], // Full passkey objects to allow signature verification
    ) -> Result<(String, String), WebAuthnError>; // (ClientJSON, StateJSON)

    async fn verify_authentication(
        &self,
        state_json: &str,
        response_json: &str,
    ) -> Result<i64, WebAuthnError>; // Returns new_sign_count
}
