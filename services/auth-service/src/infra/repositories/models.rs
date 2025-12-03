use crate::domain::models::{User, UserPlan, Passkey, OAuthProvider, UserIdentity, MagicLinkToken, Session};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserDb {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub plan: String,
    pub last_sign_in: Option<DateTime<Utc>>,
    pub last_sign_in_method: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserDb> for User {
    fn from(db: UserDb) -> Self {
        User {
            id: db.id,
            name: db.name,
            email: db.email,
            phone: db.phone,
            plan: match db.plan.as_str() {
                "Plus" => UserPlan::Plus,
                "Premium" => UserPlan::Premium,
                "Lifetime" => UserPlan::Lifetime,
                _ => UserPlan::FreeTrial,
            },
            last_sign_in: db.last_sign_in,
            last_sign_in_method: db.last_sign_in_method.and_then(|m| serde_json::from_str(&m).ok()),
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}

impl From<User> for UserDb {
    fn from(user: User) -> Self {
        UserDb {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            plan: format!("{:?}", user.plan),
            last_sign_in: user.last_sign_in,
            last_sign_in_method: user.last_sign_in_method.map(|m| serde_json::to_string(&m).unwrap_or_default()),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(FromRow)]
pub struct UserIdentityDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<UserIdentityDb> for UserIdentity {
    fn from(db: UserIdentityDb) -> Self {
        UserIdentity {
            id: db.id,
            user_id: db.user_id,
            provider: match db.provider.as_str() {
                "Apple" => OAuthProvider::Apple,
                _ => OAuthProvider::Google,
            },
            provider_id: db.provider_id,
            email: db.email,
            created_at: db.created_at,
        }
    }
}

impl From<UserIdentity> for UserIdentityDb {
    fn from(identity: UserIdentity) -> Self {
        UserIdentityDb {
            id: identity.id,
            user_id: identity.user_id,
            provider: format!("{:?}", identity.provider),
            provider_id: identity.provider_id,
            email: identity.email,
            created_at: identity.created_at,
        }
    }
}

#[derive(FromRow)]
pub struct PasskeyDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: String,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub transports: Option<String>, // JSON
    pub device_name: Option<String>,
    pub last_used_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<PasskeyDb> for Passkey {
    fn from(db: PasskeyDb) -> Self {
        Passkey {
            id: db.id,
            user_id: db.user_id,
            credential_id: db.credential_id,
            public_key: db.public_key,
            sign_count: db.sign_count,
            transports: db.transports.and_then(|t| serde_json::from_str(&t).ok()),
            device_name: db.device_name,
            last_used_at: db.last_used_at,
            created_at: db.created_at,
        }
    }
}

impl From<Passkey> for PasskeyDb {
    fn from(pk: Passkey) -> Self {
        PasskeyDb {
            id: pk.id,
            user_id: pk.user_id,
            credential_id: pk.credential_id,
            public_key: pk.public_key,
            sign_count: pk.sign_count,
            transports: pk.transports.map(|t| serde_json::to_string(&t).unwrap_or_default()),
            device_name: pk.device_name,
            last_used_at: pk.last_used_at,
            created_at: pk.created_at,
        }
    }
}

#[derive(FromRow)]
pub struct MagicLinkTokenDb {
    pub email: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

impl From<MagicLinkTokenDb> for MagicLinkToken {
    fn from(db: MagicLinkTokenDb) -> Self {
        MagicLinkToken {
            email: db.email,
            token_hash: db.token_hash,
            expires_at: db.expires_at,
            used: db.used,
        }
    }
}

impl From<MagicLinkToken> for MagicLinkTokenDb {
    fn from(token: MagicLinkToken) -> Self {
        MagicLinkTokenDb {
            email: token.email,
            token_hash: token.token_hash,
            expires_at: token.expires_at,
            used: token.used,
        }
    }
}

#[derive(FromRow)]
pub struct SessionDb {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<SessionDb> for Session {
    fn from(db: SessionDb) -> Self {
        Session {
            id: db.id,
            user_id: db.user_id,
            expires_at: db.expires_at,
            ip_address: db.ip_address,
            user_agent: db.user_agent,
            created_at: db.created_at,
        }
    }
}

impl From<Session> for SessionDb {
    fn from(session: Session) -> Self {
        SessionDb {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            created_at: session.created_at,
        }
    }
}