use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::models::*;
use crate::domain::ports::AuthRepository;
use crate::domain::errors::RepositoryError;
use super::models::*; // Import the DB models

pub struct PostgresAuthRepository {
    pool: PgPool,
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    // ==========================================
    // 1. USER MANAGEMENT
    // ==========================================

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let user_db = sqlx::query_as::<_, UserDb>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user_db.map(User::from))
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError> {
        let user_db = sqlx::query_as::<_, UserDb>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user_db.map(User::from))
    }

    async fn create_user(&self, user: User) -> Result<User, RepositoryError> {
        let db_user = UserDb::from(user.clone());
        
        sqlx::query(
            "INSERT INTO users (id, name, email, phone, plan, last_sign_in, last_sign_in_method, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(db_user.id)
        .bind(db_user.name)
        .bind(db_user.email)
        .bind(db_user.phone)
        .bind(db_user.plan)
        .bind(db_user.last_sign_in)
        .bind(db_user.last_sign_in_method)
        .bind(db_user.created_at)
        .bind(db_user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique constraint") {
                RepositoryError::DuplicateKey("User email already exists".to_string())
            } else {
                RepositoryError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<User, RepositoryError> {
        let db_user = UserDb::from(user.clone());

        sqlx::query(
            "UPDATE users SET name = $1, email = $2, phone = $3, plan = $4, last_sign_in = $5, last_sign_in_method = $6, updated_at = $7 WHERE id = $8"
        )
        .bind(db_user.name)
        .bind(db_user.email)
        .bind(db_user.phone)
        .bind(db_user.plan)
        .bind(db_user.last_sign_in)
        .bind(db_user.last_sign_in_method)
        .bind(db_user.updated_at)
        .bind(db_user.id)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // ==========================================
    // 2. OAUTH / IDENTITY MANAGEMENT
    // ==========================================

    async fn find_user_identity(&self, provider: &OAuthProvider, provider_id: &str) -> Result<Option<UserIdentity>, RepositoryError> {
        let provider_str = match provider {
            OAuthProvider::Google => "Google",
            OAuthProvider::Apple => "Apple",
        };

        let identity_db = sqlx::query_as::<_, UserIdentityDb>(
            "SELECT * FROM user_identities WHERE provider = $1 AND provider_id = $2"
        )
        .bind(provider_str)
        .bind(provider_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(identity_db.map(UserIdentity::from))
    }

    async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, RepositoryError> {
        let provider_str = match identity.provider {
            OAuthProvider::Google => "Google",
            OAuthProvider::Apple => "Apple",
        };

        sqlx::query(
            "INSERT INTO user_identities (id, user_id, provider, provider_id, email, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(identity.id)
        .bind(identity.user_id)
        .bind(provider_str)
        .bind(identity.provider_id.clone())
        .bind(identity.email.clone())
        .bind(identity.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(identity)
    }

    async fn delete_user_identities_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM user_identities WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // ==========================================
    // 3. PASSKEYS (WEBAUTHN)
    // ==========================================

    async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, RepositoryError> {
        let transports_json = passkey.transports.as_ref().map(|t| serde_json::to_string(t).unwrap_or_default());

        sqlx::query(
            "INSERT INTO passkeys (id, user_id, credential_id, public_key, sign_count, transports, device_name, last_used_at, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(passkey.id)
        .bind(passkey.user_id)
        .bind(passkey.credential_id.clone())
        .bind(passkey.public_key.clone())
        .bind(passkey.sign_count)
        .bind(transports_json)
        .bind(passkey.device_name.clone())
        .bind(passkey.last_used_at)
        .bind(passkey.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(passkey)
    }

    async fn find_passkey_by_credential_id(&self, credential_id: &str) -> Result<Option<Passkey>, RepositoryError> {
        let passkey_db = sqlx::query_as::<_, PasskeyDb>("SELECT * FROM passkeys WHERE credential_id = $1")
            .bind(credential_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(passkey_db.map(Passkey::from))
    }

    async fn find_passkeys_by_user_id(&self, user_id: Uuid) -> Result<Vec<Passkey>, RepositoryError> {
        let passkeys_db = sqlx::query_as::<_, PasskeyDb>("SELECT * FROM passkeys WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(passkeys_db.into_iter().map(Passkey::from).collect())
    }

    async fn update_passkey_sign_count(&self, credential_id: &str, new_sign_count: i64) -> Result<(), RepositoryError> {
        sqlx::query("UPDATE passkeys SET sign_count = $1, last_used_at = $2 WHERE credential_id = $3")
            .bind(new_sign_count)
            .bind(Utc::now())
            .bind(credential_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete_passkeys_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM passkeys WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // ==========================================
    // 4. MAGIC LINKS
    // ==========================================

    async fn save_magic_link(&self, magic_link: MagicLinkToken) -> Result<(), RepositoryError> {
        sqlx::query(
            "INSERT INTO magic_links (email, token_hash, expires_at, used) VALUES ($1, $2, $3, $4)"
        )
        .bind(magic_link.email.clone())
        .bind(magic_link.token_hash.clone())
        .bind(magic_link.expires_at)
        .bind(magic_link.used)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_magic_link_by_email(&self, email: &str) -> Result<Option<MagicLinkToken>, RepositoryError> {
        let token_db = sqlx::query_as::<_, MagicLinkTokenDb>(
            "SELECT * FROM magic_links WHERE email = $1 AND used = false AND expires_at > $2 ORDER BY expires_at DESC LIMIT 1"
        )
        .bind(email)
        .bind(Utc::now())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(token_db.map(MagicLinkToken::from))
    }

    async fn mark_magic_link_as_used(&self, email: &str) -> Result<(), RepositoryError> {
        sqlx::query("UPDATE magic_links SET used = true WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete_magic_links_for_email(&self, email: &str) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM magic_links WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete_expired_magic_links(&self) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM magic_links WHERE expires_at < $1")
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // ==========================================
    // 5. SESSIONS
    // ==========================================

    async fn create_session(&self, session: Session) -> Result<Session, RepositoryError> {
        let db_session = SessionDb::from(session.clone());
        sqlx::query(
            "INSERT INTO sessions (id, user_id, expires_at, ip_address, user_agent, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(db_session.id)
        .bind(db_session.user_id)
        .bind(db_session.expires_at)
        .bind(db_session.ip_address)
        .bind(db_session.user_agent)
        .bind(db_session.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(session)
    }

    async fn find_session_by_id(&self, session_id: &str) -> Result<Option<Session>, RepositoryError> {
        let session_db = sqlx::query_as::<_, SessionDb>("SELECT * FROM sessions WHERE id = $1")
            .bind(session_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(session_db.map(Session::from))
    }

    async fn update_session(&self, session: Session) -> Result<Session, RepositoryError> {
        sqlx::query("UPDATE sessions SET expires_at = $1 WHERE id = $2")
            .bind(session.expires_at)
            .bind(session.id.clone())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(session)
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete_expired_sessions(&self) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM sessions WHERE expires_at < $1")
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
