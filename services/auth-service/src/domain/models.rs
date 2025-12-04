use uuid::Uuid;
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};

// ==========================================
// 1. THE CORE IDENTITY (Who they are)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,                  // Stored in E.164 format (e.g., "+14155552671") optional because they might sign up via Email only first.
    pub plan: UserPlan,                         // The "Tier" controls access to features in SvelteKit
    pub last_sign_in: Option<DateTime<Utc>>,    // Auditing / Security Snapshots
    pub last_sign_in_method: Option<LastSignInMethod>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserPlan {
    FreeTrial,
    Plus,
    Premium,
    Lifetime,
}

impl User {
    pub fn update_plan(&mut self, new_plan: UserPlan) {
        self.plan = new_plan;
        self.updated_at = Utc::now();
    }

    /// Check if the user's plan is currently active.
    /// For now, all plans are considered active.
    /// In the future, this could check subscription expiry dates.
    pub fn is_plan_active(&self) -> bool {
        true 
    }

    /// Check if the user is allowed to register a new passkey.
    /// This could limit the number of passkeys based on the plan.
    pub fn can_register_passkey(&self, current_passkey_count: usize) -> bool {
        match self.plan {
            UserPlan::FreeTrial => current_passkey_count < 2,
            _ => true, // Paid plans can have unlimited passkeys (or a much higher limit)
        }
    }
}

// ==========================================
// 2. THE OAUTH LINKS (Google / Apple)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,                          // Foreign Key to User
    pub provider: OAuthProvider,                // Google, Apple
    pub provider_id: String,                    // The unique ID sent by the provider (e.g., Google "sub" claim)
    pub email: String,                          // The email specific to this provider (might differ from User.email)
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OAuthProvider {
    Google,
    Apple,
}

// ==========================================
// 3. THE PASSKEYS (WebAuthn / Biometrics)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passkey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: String,                  // The Base64URL ID used by the browser to find the credential
    pub public_key: Vec<u8>,                    // The raw public key bytes (Stored as BYTEA/BLOB)
    pub sign_count: i64,                        // CRITICAL: Counter to prevent Replay Attacks. (If incoming count <= stored count, reject SignIn)
    pub transports: Option<Vec<String>>,        // hints: ["internal", "hybrid"] -> "FaceID" vs "USB Key"
    pub device_name: Option<String>,
    pub last_used_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Passkey {
    /// Verify that the incoming signature counter is valid (strictly increasing).
    /// This prevents replay attacks where an attacker captures a valid assertion and tries to use it again.
    pub fn verify_sign_count(&self, new_sign_count: i64) -> bool {
        // If the stored counter is 0 and incoming is 0, it might be a device that doesn't support counters.
        // However, standard behavior is strictly increasing.
        // NOTE: Some authenticators always return 0. We should handle that policy-wise, 
        // but for strict security, > is required.
        new_sign_count > self.sign_count
    }
}

// ==========================================
// 4. THE MAGIC LINKS (Email SignIn)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicLinkToken {
    pub email: String,                          // We use Email as the key because the User might not exist yet!
    pub token_hash: String,                     // SECURITY: Store the SHA256 Hash, NOT the raw token.
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

impl MagicLinkToken {
    pub fn is_valid(&self) -> bool {
        !self.used && self.expires_at > Utc::now()
    }
}

// ==========================================
// 5. THE ACTIVE SESSION (The "Cookie")
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,                             // This is the browser cookie
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Refresh the session expiration if it is close to expiring.
    /// Returns true if the session was refreshed (and needs saving).
    pub fn refresh_if_needed(&mut self, threshold_days: i64, extend_days: i64) -> bool {
        let days_remaining = (self.expires_at - Utc::now()).num_days();
        if days_remaining < threshold_days {
            self.expires_at = Utc::now() + Duration::days(extend_days);
            return true;
        }
        false
    }
}

#[derive(Debug, Serialize)]
pub struct SessionUser {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub plan: UserPlan,
    pub avatar_url: Option<String>,
}

// ==========================================
// 6. SHARED ENUMS
// ==========================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LastSignInMethod {
    OAuth(OAuthProvider),
    MagicLink,
    Passkey,
}