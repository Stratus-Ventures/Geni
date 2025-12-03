use std::sync::Mutex;
use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::models::*;
use crate::domain::ports::{AuthRepository, ChallengeStore, WebAuthnVerifier};
use crate::domain::errors::{RepositoryError, ChallengeStoreError, WebAuthnError};

// ==========================================
// MOCK AUTH REPOSITORY
// ==========================================

pub struct MockAuthRepository {
    pub users: Mutex<HashMap<Uuid, User>>,
    pub users_by_email: Mutex<HashMap<String, Uuid>>,
    pub passkeys: Mutex<HashMap<String, Passkey>>,
    pub magic_links: Mutex<HashMap<String, MagicLinkToken>>,
    pub sessions: Mutex<HashMap<String, Session>>,
    pub identities: Mutex<HashMap<(OAuthProvider, String), UserIdentity>>,
}

impl MockAuthRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            users_by_email: Mutex::new(HashMap::new()),
            passkeys: Mutex::new(HashMap::new()),
            magic_links: Mutex::new(HashMap::new()),
            sessions: Mutex::new(HashMap::new()),
            identities: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AuthRepository for MockAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let map = self.users_by_email.lock().unwrap();
        if let Some(id) = map.get(email) {
            let users = self.users.lock().unwrap();
            Ok(users.get(id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&user_id).cloned())
    }

    async fn create_user(&self, user: User) -> Result<User, RepositoryError> {
        let mut users = self.users.lock().unwrap();
        let mut map = self.users_by_email.lock().unwrap();
        if map.contains_key(&user.email) {
            return Err(RepositoryError::DuplicateKey("Email exists".into()));
        }
        map.insert(user.email.clone(), user.id);
        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<User, RepositoryError> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        let mut users = self.users.lock().unwrap();
        if let Some(u) = users.remove(&user_id) {
            let mut map = self.users_by_email.lock().unwrap();
            map.remove(&u.email);
        }
        Ok(())
    }

    // Passkeys
    async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError> {
        let mut pk = self.passkeys.lock().unwrap();
        pk.insert(passkey.credential_id.clone(), passkey.clone());
        Ok(passkey)
    }

    async fn find_passkey_by_credential_id(&self, credential_id: &str) -> Result<Option<Passkey>, RepositoryError> {
        let pk = self.passkeys.lock().unwrap();
        Ok(pk.get(credential_id).cloned())
    }

    async fn find_passkeys_by_user_id(&self, user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError> {
        let pk = self.passkeys.lock().unwrap();
        Ok(pk.values().filter(|p| p.user_id == user_id).cloned().collect())
    }

    async fn update_passkey_sign_count(&self, credential_id: &str, new_sign_count: i64) -> Result<(), RepositoryError> {
        let mut pk = self.passkeys.lock().unwrap();
        if let Some(p) = pk.get_mut(credential_id) {
            p.sign_count = new_sign_count;
        }
        Ok(())
    }

    async fn delete_passkeys_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        let mut pk = self.passkeys.lock().unwrap();
        pk.retain(|_, p| p.user_id != user_id);
        Ok(())
    }

    // Magic Links
    async fn save_magic_link(&self, magic_link: MagicLinkToken) -> Result<(), RepositoryError> {
        let mut ml = self.magic_links.lock().unwrap();
        ml.insert(magic_link.email.clone(), magic_link);
        Ok(())
    }

    async fn find_magic_link_by_email(&self, email: &str) -> Result<Option<MagicLinkToken>, RepositoryError> {
        let ml = self.magic_links.lock().unwrap();
        Ok(ml.get(email).cloned())
    }

    async fn mark_magic_link_as_used(&self, email: &str) -> Result<(), RepositoryError> {
        let mut ml = self.magic_links.lock().unwrap();
        if let Some(m) = ml.get_mut(email) {
            m.used = true;
        }
        Ok(())
    }

    async fn delete_magic_links_for_email(&self, email: &str) -> Result<(), RepositoryError> {
        let mut ml = self.magic_links.lock().unwrap();
        ml.remove(email);
        Ok(())
    }

    async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> {
        Ok(())
    }

    // Sessions
    async fn create_session(&self, session: Session) -> Result<Session, RepositoryError> {
        let mut s = self.sessions.lock().unwrap();
        s.insert(session.id.clone(), session.clone());
        Ok(session)
    }

    async fn find_session_by_id(&self, session_id: &str) -> Result<Option<Session>, RepositoryError> {
        let s = self.sessions.lock().unwrap();
        Ok(s.get(session_id).cloned())
    }

    async fn update_session(&self, session: Session) -> Result<Session, RepositoryError> {
        let mut s = self.sessions.lock().unwrap();
        s.insert(session.id.clone(), session.clone());
        Ok(session)
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), RepositoryError> {
        let mut s = self.sessions.lock().unwrap();
        s.remove(session_id);
        Ok(())
    }

    async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        let mut s = self.sessions.lock().unwrap();
        s.retain(|_, sess| sess.user_id != user_id);
        Ok(())
    }

    async fn delete_expired_sessions(&self) -> Result<(), RepositoryError> {
        Ok(())
    }

    // OAuth
    async fn find_user_identity(&self, provider: &OAuthProvider, provider_id: &str) -> Result<Option<UserIdentity>, RepositoryError> {
        let ids = self.identities.lock().unwrap();
        Ok(ids.get(&(provider.clone(), provider_id.to_string())).cloned())
    }

    async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, RepositoryError> {
        let mut ids = self.identities.lock().unwrap();
        ids.insert((identity.provider.clone(), identity.provider_id.clone()), identity.clone());
        Ok(identity)
    }

    async fn delete_user_identities_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        let mut ids = self.identities.lock().unwrap();
        ids.retain(|_, i| i.user_id != user_id);
        Ok(())
    }
}

// ==========================================
// MOCK CHALLENGE STORE
// ==========================================

pub struct MockChallengeStore {
    pub challenges: Mutex<HashMap<String, String>>,
}

impl MockChallengeStore {
    pub fn new() -> Self {
        Self { challenges: Mutex::new(HashMap::new()) }
    }
}

#[async_trait]
impl ChallengeStore for MockChallengeStore {
    async fn store_challenge(&self, key: &str, challenge: &str, _ttl: u64) -> Result<(), ChallengeStoreError> {
        let mut c = self.challenges.lock().unwrap();
        c.insert(key.to_string(), challenge.to_string());
        Ok(())
    }

    async fn get_challenge(&self, key: &str) -> Result<Option<String>, ChallengeStoreError> {
        let c = self.challenges.lock().unwrap();
        Ok(c.get(key).cloned())
    }

    async fn delete_challenge(&self, key: &str) -> Result<(), ChallengeStoreError> {
        let mut c = self.challenges.lock().unwrap();
        c.remove(key);
        Ok(())
    }
}

// ==========================================
// MOCK WEBAUTHN VERIFIER
// ==========================================

pub struct MockWebAuthnVerifier;

impl MockWebAuthnVerifier {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl WebAuthnVerifier for MockWebAuthnVerifier {
    async fn start_registration(&self, _user_id: Uuid, _email: &str, _name: &str) -> Result<(String, String), WebAuthnError> {
        Ok(("{\"challenge\":\"mock_reg_challenge\"}".to_string(), "{\"state\":\"mock_reg_state\"}".to_string()))
    }

    async fn verify_registration(&self, _state: &str, _response: &str) -> Result<(String, Vec<u8>), WebAuthnError> {
        Ok(("cred_123".to_string(), vec![1, 2, 3, 4]))
    }

    async fn start_authentication(&self, _creds: &[Passkey]) -> Result<(String, String), WebAuthnError> {
        Ok(("{\"challenge\":\"mock_auth_challenge\"}".to_string(), "{\"state\":\"mock_auth_state\"}".to_string()))
    }

    async fn verify_authentication(&self, _state: &str, _response: &str) -> Result<i64, WebAuthnError> {
        Ok(10)
    }
}
