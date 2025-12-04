use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::application::app_error::{AppError, AppResult};
use crate::domain::models::*;
use crate::domain::ports::AuthRepository;

/// The UserService handles user profile management and administrative operations.
/// This includes updating user plans, profile information, and account settings.
pub struct UserService {
    repo: Arc<dyn AuthRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    /// Update a user's subscription plan.
    ///
    /// **Flow:**
    /// 1. Find the user by ID.
    /// 2. Update their plan (FreeTrial -> Plus -> Premium -> Lifetime).
    /// 3. Save the updated user to the database.
    ///
    /// **Use Cases:**
    /// - Stripe webhook: User completed payment -> Upgrade to Plus/Premium.
    /// - Admin action: Grant lifetime access to a beta tester.
    /// - Subscription expired: Downgrade Premium -> FreeTrial.
    /// - Lifetime promo: User purchased lifetime access.
    ///
    /// **Security:** This should be called from:
    /// - Stripe webhook handler (verified signature).
    /// - Admin API endpoint (requires admin role).
    /// - Background job processing subscription renewals.
    pub async fn update_plan(&self, user_id: Uuid, new_plan: UserPlan) -> AppResult<User> {
        // Find the user
        let mut user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Update the plan
        user.plan = new_plan;
        user.updated_at = Utc::now();

        // Save to database
        let updated_user = self
            .repo
            .update_user(user)
            .await?;

        Ok(updated_user)
    }

    /// Update a user's profile information (name, phone).
    ///
    /// **Flow:**
    /// 1. Find the user by ID.
    /// 2. Update their name and/or phone number.
    /// 3. Validate phone number format (E.164).
    /// 4. Save the updated user to the database.
    ///
    /// **Use Case:** User edits their profile in account settings.
    ///
    /// **Security:** This should only be called when the user is authenticated
    /// (the API layer verifies the session and ensures user_id matches the session).
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        name: Option<String>,
        phone: Option<String>,
    ) -> AppResult<User> {
        // Find the user
        let mut user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Update name if provided
        if let Some(new_name) = name {
            let trimmed_name = new_name.trim();
            if trimmed_name.is_empty() {
                return Err(AppError::Internal("Name cannot be empty".to_string()));
            }
            if trimmed_name.len() > 100 {
                return Err(AppError::Internal("Name is too long (max 100 characters)".to_string()));
            }
            user.name = trimmed_name.to_string();
        }

        // Update phone if provided
        if let Some(new_phone) = phone {
            let trimmed_phone = new_phone.trim();
            if !trimmed_phone.is_empty() {
                // Clean and normalize the phone number to E.164 format
                let cleaned_phone = clean_phone_number(trimmed_phone)
                    .map_err(|e| AppError::Internal(format!("Invalid phone number: {}", e)))?;
                user.phone = Some(cleaned_phone);
            } else {
                // Empty string means remove phone number
                user.phone = None;
            }
        }

        user.updated_at = Utc::now();

        // Save to database
        let updated_user = self
            .repo
            .update_user(user)
            .await?;

        Ok(updated_user)
    }

    /// Get a user's full profile information.
    ///
    /// **Use Case:** Display user profile in account settings, admin panel, etc.
    pub async fn get_user_profile(&self, user_id: Uuid) -> AppResult<User> {
        self.repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)
    }

    /// Update a user's email address.
    ///
    /// **Flow:**
    /// 1. Verify the new email is not already in use.
    /// 2. Update the user's email.
    /// 3. For security: Log out all devices (force re-authentication).
    ///
    /// **Security Considerations:**
    /// - Email change is a critical security operation.
    /// - Should require email verification (send confirmation to BOTH old and new email).
    /// - Should revoke all sessions to prevent session hijacking.
    ///
    /// **Use Case:** User changes their primary email address in account settings.
    pub async fn update_email(
        &self,
        user_id: Uuid,
        new_email: String,
    ) -> AppResult<User> {
        let normalized_email = new_email.to_lowercase().trim().to_string();

        // Validate email format
        if !is_valid_email(&normalized_email) {
            return Err(AppError::Internal("Invalid email format".to_string()));
        }

        // Check if the email is already in use by another user
        if let Some(existing_user) = self
            .repo
            .find_user_by_email(&normalized_email)
            .await?
        {
            if existing_user.id != user_id {
                return Err(AppError::Internal("Email is already in use".to_string()));
            }
        }

        // Find the user
        let mut user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Update the email
        user.email = normalized_email;
        user.updated_at = Utc::now();

        // Save to database
        let updated_user = self
            .repo
            .update_user(user)
            .await?;

        // For security: Revoke all sessions (log out all devices)
        // The user will need to log in again with their new email
        self.repo
            .delete_all_sessions_for_user(user_id)
            .await?;

        Ok(updated_user)
    }

    /// Delete a user account (GDPR compliance).
    ///
    /// **Flow:**
    /// 1. Delete all sessions (log out all devices).
    /// 2. Delete all OAuth identities.
    /// 3. Delete all passkeys.
    /// 4. Delete all magic links.
    /// 5. Optionally: Soft-delete the user (mark as deleted) vs hard-delete.
    ///
    /// **GDPR Compliance:** Users have the "right to be forgotten".
    /// This function should remove all personal data.
    ///
    /// **Use Case:** User clicks "Delete my account" in settings.
    ///
    /// **Note:** This is a simplified implementation. In production, you might:
    /// - Soft-delete (mark user as deleted but keep data for audit).
    /// - Schedule deletion (grace period of 30 days).
    /// - Anonymize data instead of deleting (for analytics).
    pub async fn delete_account(&self, user_id: Uuid) -> AppResult<()> {
        // Find user to get email (needed for magic link deletion)
        let user = self
            .repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::InvalidCredentials)?;

        // Cascade deletion in proper order (respecting foreign key dependencies)
        // Order matters: delete dependent records first, then the user record

        // 1. Delete all sessions (user can no longer authenticate)
        self.repo
            .delete_all_sessions_for_user(user_id)
            .await?;

        // 2. Delete all OAuth identities (Google/Apple links)
        self.repo
            .delete_user_identities_for_user(user_id)
            .await?;

        // 3. Delete all passkeys (WebAuthn credentials)
        self.repo
            .delete_passkeys_for_user(user_id)
            .await?;

        // 4. Delete all magic link tokens
        self.repo
            .delete_magic_links_for_email(&user.email)
            .await?;

        // 5. Finally, delete the user record itself
        self.repo
            .delete_user(user_id)
            .await?;

        Ok(())
    }
}

// ==========================================
// HELPER FUNCTIONS
// ==========================================

/// Validate email format (simplified).
/// For production, use the `validator` or `email_address` crate.
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

/// Clean and normalize a phone number to E.164 format.
///
/// **Examples:**
/// - "(415) 555-2671" -> "+14155552671" (assumes US if no country code)
/// - "555-555-5555" -> "+15555555555" (assumes US)
/// - "+44 7911 123456" -> "+447911123456" (preserves country code)
/// - "1-415-555-2671" -> "+14155552671"
///
/// **Note:** This is a simplified implementation that assumes US (+1) as the default country.
/// For production, use the `phone-number` crate for proper international support.
fn clean_phone_number(phone: &str) -> Result<String, String> {
    // Remove all non-digit characters except leading '+'
    let has_plus = phone.starts_with('+');
    let digits_only: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits_only.is_empty() {
        return Err("Phone number contains no digits".to_string());
    }

    let formatted = if has_plus {
        // Already has a country code prefix
        format!("+{}", digits_only)
    } else if digits_only.len() == 10 {
        // US phone number without country code (e.g., 4155552671)
        format!("+1{}", digits_only)
    } else if digits_only.len() == 11 && digits_only.starts_with('1') {
        // US phone number with '1' prefix (e.g., 14155552671)
        format!("+{}", digits_only)
    } else if digits_only.len() < 7 {
        return Err("Phone number too short".to_string());
    } else if digits_only.len() > 15 {
        return Err("Phone number too long".to_string());
    } else {
        // Assume it already includes country code
        format!("+{}", digits_only)
    };

    // Validate the result
    if is_valid_e164_phone(&formatted) {
        Ok(formatted)
    } else {
        Err("Invalid phone number format".to_string())
    }
}

/// Validate E.164 phone number format.
/// E.164 format: +[country code][number]
/// Examples: +14155552671 (US), +447911123456 (UK), +81312345678 (Japan)
///
/// For production, use the `phone-number` crate for proper validation.
fn is_valid_e164_phone(phone: &str) -> bool {
    // Basic validation: starts with '+', followed by 7-15 digits
    if !phone.starts_with('+') {
        return false;
    }

    let digits = &phone[1..];
    if digits.len() < 7 || digits.len() > 15 {
        return false;
    }

    digits.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // ==========================================
    // MOCK REPOSITORY FOR TESTING
    // ==========================================

    struct MockAuthRepository {
        users: Mutex<HashMap<Uuid, User>>,
        users_by_email: Mutex<HashMap<String, Uuid>>,
        sessions: Mutex<HashMap<String, Session>>,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
                users_by_email: Mutex::new(HashMap::new()),
                sessions: Mutex::new(HashMap::new()),
            }
        }

        fn with_user(user: User) -> Self {
            let mut users = HashMap::new();
            let mut users_by_email = HashMap::new();
            users_by_email.insert(user.email.clone(), user.id);
            users.insert(user.id, user);

            Self {
                users: Mutex::new(users),
                users_by_email: Mutex::new(users_by_email),
                sessions: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::domain::ports::AuthRepository for MockAuthRepository {
        async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, crate::domain::errors::RepositoryError> {
            let users_by_email = self.users_by_email.lock().unwrap();
            if let Some(user_id) = users_by_email.get(email) {
                let users = self.users.lock().unwrap();
                Ok(users.get(user_id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn find_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, crate::domain::errors::RepositoryError> {
            let users = self.users.lock().unwrap();
            Ok(users.get(&user_id).cloned())
        }

        async fn create_user(&self, user: User) -> Result<User, crate::domain::errors::RepositoryError> {
            let mut users = self.users.lock().unwrap();
            let mut users_by_email = self.users_by_email.lock().unwrap();
            users_by_email.insert(user.email.clone(), user.id);
            users.insert(user.id, user.clone());
            Ok(user)
        }

        async fn update_user(&self, user: User) -> Result<User, crate::domain::errors::RepositoryError> {
            let mut users = self.users.lock().unwrap();
            let mut users_by_email = self.users_by_email.lock().unwrap();

            // Update email index if email changed
            if let Some(old_user) = users.get(&user.id) {
                if old_user.email != user.email {
                    users_by_email.remove(&old_user.email);
                    users_by_email.insert(user.email.clone(), user.id);
                }
            }

            users.insert(user.id, user.clone());
            Ok(user)
        }

        async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), crate::domain::errors::RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.retain(|_, session| session.user_id != user_id);
            Ok(())
        }

        // Minimal implementations for other required trait methods
        async fn find_user_identity(&self, _provider: &OAuthProvider, _provider_id: &str) -> Result<Option<UserIdentity>, crate::domain::errors::RepositoryError> {
            Ok(None)
        }
        async fn create_user_identity(&self, identity: UserIdentity) -> Result<UserIdentity, crate::domain::errors::RepositoryError> {
            Ok(identity)
        }
        async fn save_passkey(&self, passkey: Passkey) -> Result<Passkey, crate::domain::errors::RepositoryError> {
            Ok(passkey)
        }
        async fn find_passkey_by_credential_id(&self, _credential_id: &str) -> Result<Option<Passkey>, crate::domain::errors::RepositoryError> {
            Ok(None)
        }
        async fn find_passkeys_by_user_id(&self, _user_id: Uuid) -> Result<Vec<Passkey>, crate::domain::errors::RepositoryError> {
            Ok(Vec::new())
        }
        async fn update_passkey_sign_count(&self, _credential_id: &str, _new_sign_count: i64) -> Result<(), crate::domain::errors::RepositoryError> {
            Ok(())
        }
        async fn save_magic_link(&self, _magic_link: MagicLinkToken) -> Result<(), crate::domain::errors::RepositoryError> {
            Ok(())
        }
        async fn find_magic_link_by_email(&self, _email: &str) -> Result<Option<MagicLinkToken>, crate::domain::errors::RepositoryError> {
            Ok(None)
        }
        async fn mark_magic_link_as_used(&self, _email: &str) -> Result<(), crate::domain::errors::RepositoryError> {
            Ok(())
        }
        async fn delete_expired_magic_links(&self) -> Result<(), crate::domain::errors::RepositoryError> {
            Ok(())
        }
        async fn create_session(&self, session: Session) -> Result<Session, crate::domain::errors::RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }
        async fn find_session_by_id(&self, session_id: &str) -> Result<Option<Session>, crate::domain::errors::RepositoryError> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.get(session_id).cloned())
        }
        async fn update_session(&self, session: Session) -> Result<Session, crate::domain::errors::RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }
        async fn delete_session(&self, session_id: &str) -> Result<(), crate::domain::errors::RepositoryError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(session_id);
            Ok(())
        }
        async fn delete_expired_sessions(&self) -> Result<(), crate::domain::errors::RepositoryError> {
            Ok(())
        }

        // Cascade deletion methods
        async fn delete_passkeys_for_user(&self, _user_id: Uuid) -> Result<(), crate::domain::errors::RepositoryError> {
            // Minimal implementation for testing
            Ok(())
        }

        async fn delete_user_identities_for_user(&self, _user_id: Uuid) -> Result<(), crate::domain::errors::RepositoryError> {
            // Minimal implementation for testing
            Ok(())
        }

        async fn delete_magic_links_for_email(&self, _email: &str) -> Result<(), crate::domain::errors::RepositoryError> {
            // Minimal implementation for testing
            Ok(())
        }

        async fn delete_user(&self, user_id: Uuid) -> Result<(), crate::domain::errors::RepositoryError> {
            let mut users = self.users.lock().unwrap();
            let mut users_by_email = self.users_by_email.lock().unwrap();

            // Remove from email index
            if let Some(user) = users.get(&user_id) {
                users_by_email.remove(&user.email);
            }

            // Remove user
            users.remove(&user_id);
            Ok(())
        }
    }

    // ==========================================
    // HELPER FUNCTIONS FOR TESTS
    // ==========================================

    fn create_test_user(email: &str) -> User {
        User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: email.to_string(),
            phone: None,
            plan: UserPlan::FreeTrial,
            last_sign_in: None,
            last_sign_in_method: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // ==========================================
    // UNIT TESTS FOR UserService METHODS
    // ==========================================

    #[tokio::test]
    async fn test_update_plan() {
        dbg!("\n✨ TEST: test_update_plan ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo.clone());

        // Update plan from FreeTrial to Plus
        dbg!("  ➡️  Action: Updating plan to Plus");
        let updated = service.update_plan(user_id, UserPlan::Plus).await.unwrap();
        assert_eq!(updated.plan, UserPlan::Plus);
        dbg!("  ✅  Result: Plan updated to Plus");

        // Update plan to Premium
        dbg!("  ➡️  Action: Updating plan to Premium");
        let updated = service.update_plan(user_id, UserPlan::Premium).await.unwrap();
        assert_eq!(updated.plan, UserPlan::Premium);
        dbg!("  ✅  Result: Plan updated to Premium");

        // Update plan to Lifetime
        dbg!("  ➡️  Action: Updating plan to Lifetime");
        let updated = service.update_plan(user_id, UserPlan::Lifetime).await.unwrap();
        assert_eq!(updated.plan, UserPlan::Lifetime);
        dbg!("  ✅  Result: Plan updated to Lifetime");
    }

    #[tokio::test]
    async fn test_update_plan_nonexistent_user() {
        dbg!("\n✨ TEST: test_update_plan_nonexistent_user ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = UserService::new(repo);

        dbg!("  ➡️  Action: Updating plan for non-existent user");
        let result = service.update_plan(Uuid::new_v4(), UserPlan::Plus).await;
        assert!(result.is_err());
        dbg!("  ✅  Result: Error returned as expected");
    }

    #[tokio::test]
    async fn test_update_profile_name() {
        dbg!("\n✨ TEST: test_update_profile_name ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Update name
        dbg!("  ➡️  Action: Updating name to 'John Doe'");
        let updated = service
            .update_profile(user_id, Some("John Doe".to_string()), None)
            .await
            .unwrap();

        assert_eq!(updated.name, "John Doe");
        dbg!("  ✅  Result: Name updated successfully");
    }

    #[tokio::test]
    async fn test_update_profile_phone() {
        dbg!("\n✨ TEST: test_update_profile_phone ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Update phone with various formats
        dbg!("  ➡️  Action: Updating phone to '(415) 555-2671'");
        let updated = service
            .update_profile(user_id, None, Some("(415) 555-2671".to_string()))
            .await
            .unwrap();

        assert_eq!(updated.phone, Some("+14155552671".to_string()));
        dbg!("  ✅  Result: Phone updated and normalized to +14155552671");
    }

    #[tokio::test]
    async fn test_update_profile_remove_phone() {
        dbg!("\n✨ TEST: test_update_profile_remove_phone ✨");
        let mut user = create_test_user("test@example.com");
        user.phone = Some("+14155552671".to_string());
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Remove phone by passing empty string
        dbg!("  ➡️  Action: Removing phone number (empty string)");
        let updated = service
            .update_profile(user_id, None, Some("".to_string()))
            .await
            .unwrap();

        assert_eq!(updated.phone, None);
        dbg!("  ✅  Result: Phone number removed");
    }

    #[tokio::test]
    async fn test_update_profile_invalid_phone() {
        dbg!("\n✨ TEST: test_update_profile_invalid_phone ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Try to update with invalid phone
        dbg!("  ➡️  Action: Updating with invalid phone 'invalid'");
        let result = service
            .update_profile(user_id, None, Some("invalid".to_string()))
            .await;

        assert!(result.is_err());
        dbg!("  ✅  Result: Invalid phone rejected");
    }

    #[tokio::test]
    async fn test_update_profile_name_too_long() {
        dbg!("\n✨ TEST: test_update_profile_name_too_long ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Try to update with name that's too long (>100 chars)
        dbg!("  ➡️  Action: Updating with name > 100 chars");
        let long_name = "a".repeat(101);
        let result = service
            .update_profile(user_id, Some(long_name), None)
            .await;

        assert!(result.is_err());
        dbg!("  ✅  Result: Long name rejected");
    }

    #[tokio::test]
    async fn test_update_profile_empty_name() {
        dbg!("\n✨ TEST: test_update_profile_empty_name ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Try to update with empty name
        dbg!("  ➡️  Action: Updating with empty/whitespace name");
        let result = service
            .update_profile(user_id, Some("   ".to_string()), None)
            .await;

        assert!(result.is_err());
        dbg!("  ✅  Result: Empty name rejected");
    }

    #[tokio::test]
    async fn test_get_user_profile() {
        dbg!("\n✨ TEST: test_get_user_profile ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user.clone()));
        let service = UserService::new(repo);

        dbg!("  ➡️  Action: Getting user profile");
        let profile = service.get_user_profile(user_id).await.unwrap();
        assert_eq!(profile.id, user.id);
        assert_eq!(profile.email, user.email);
        dbg!("  ✅  Result: Profile retrieved correctly");
    }

    #[tokio::test]
    async fn test_get_user_profile_not_found() {
        dbg!("\n✨ TEST: test_get_user_profile_not_found ✨");
        let repo = Arc::new(MockAuthRepository::new());
        let service = UserService::new(repo);

        dbg!("  ➡️  Action: Getting non-existent profile");
        let result = service.get_user_profile(Uuid::new_v4()).await;
        assert!(result.is_err());
        dbg!("  ✅  Result: Error returned as expected");
    }

    #[tokio::test]
    async fn test_update_email() {
        dbg!("\n✨ TEST: test_update_email ✨");
        let user = create_test_user("old@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo.clone());

        // Update email
        dbg!("  ➡️  Action: Updating email to 'new@example.com'");
        let updated = service
            .update_email(user_id, "new@example.com".to_string())
            .await
            .unwrap();

        assert_eq!(updated.email, "new@example.com");

        // Verify old email is no longer found
        let result = repo.find_user_by_email("old@example.com").await.unwrap();
        assert!(result.is_none());

        // Verify new email is found
        let result = repo.find_user_by_email("new@example.com").await.unwrap();
        assert!(result.is_some());
        dbg!("  ✅  Result: Email updated and indexes refreshed");
    }

    #[tokio::test]
    async fn test_update_email_invalid_format() {
        dbg!("\n✨ TEST: test_update_email_invalid_format ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Try to update with invalid email
        dbg!("  ➡️  Action: Updating with invalid email");
        let result = service.update_email(user_id, "invalid".to_string()).await;
        assert!(result.is_err());
        dbg!("  ✅  Result: Invalid email rejected");
    }

    #[tokio::test]
    async fn test_update_email_already_exists() {
        dbg!("\n✨ TEST: test_update_email_already_exists ✨");
        let user1 = create_test_user("user1@example.com");
        let user2 = create_test_user("user2@example.com");
        let user1_id = user1.id;

        let repo = Arc::new(MockAuthRepository::new());
        repo.create_user(user1).await.unwrap();
        repo.create_user(user2).await.unwrap();

        let service = UserService::new(repo);

        // Try to update user1's email to user2's email
        dbg!("  ➡️  Action: Updating email to one that already exists");
        let result = service
            .update_email(user1_id, "user2@example.com".to_string())
            .await;

        assert!(result.is_err());
        dbg!("  ✅  Result: Duplicate email rejected");
    }

    #[tokio::test]
    async fn test_update_email_normalizes_case() {
        dbg!("\n✨ TEST: test_update_email_normalizes_case ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo);

        // Email should be normalized to lowercase
        dbg!("  ➡️  Action: Updating email with uppercase chars");
        let updated = service
            .update_email(user_id, "NEW@EXAMPLE.COM".to_string())
            .await
            .unwrap();

        assert_eq!(updated.email, "new@example.com");
        dbg!("  ✅  Result: Email normalized to lowercase");
    }

    #[tokio::test]
    async fn test_delete_account() {
        dbg!("\n✨ TEST: test_delete_account ✨");
        let user = create_test_user("test@example.com");
        let user_id = user.id;

        let repo = Arc::new(MockAuthRepository::with_user(user));
        let service = UserService::new(repo.clone());

        // Create a session for the user
        let session = Session {
            id: "test_session".to_string(),
            user_id,
            expires_at: Utc::now() + chrono::Duration::days(1),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };
        repo.create_session(session).await.unwrap();

        // Delete account
        dbg!("  ➡️  Action: Deleting account");
        service.delete_account(user_id).await.unwrap();

        // Verify session was deleted
        let session_result = repo.find_session_by_id("test_session").await.unwrap();
        assert!(session_result.is_none());

        // Verify user was deleted
        let user_result = repo.find_user_by_id(user_id).await.unwrap();
        assert!(user_result.is_none());

        // Verify user cannot be found by email
        let email_result = repo.find_user_by_email("test@example.com").await.unwrap();
        assert!(email_result.is_none());
        dbg!("  ✅  Result: Account and associated data deleted");
    }

    // ==========================================
    // HELPER FUNCTION TESTS
    // ==========================================

    #[test]
    fn test_email_validation() {
        dbg!("\n✨ TEST: test_email_validation ✨");
        // Valid emails
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("test.user+filter@domain.co.uk"));
        assert!(is_valid_email("a@b.c"));
        assert!(is_valid_email("user.name@company.org"));

        // Invalid emails
        assert!(!is_valid_email("invalid"));           // No @
        assert!(!is_valid_email("@example.com"));      // Empty local part
        assert!(!is_valid_email("user@"));             // Empty domain
        assert!(!is_valid_email("user@domain"));       // No dot in domain
        assert!(!is_valid_email("user@@domain.com"));  // Double @
        assert!(!is_valid_email("user@.domain.com"));  // Domain starts with dot
        assert!(!is_valid_email("user@domain.com."));  // Domain ends with dot
        assert!(!is_valid_email("@"));                 // Just @
        assert!(!is_valid_email("ab"));                // Too short
        dbg!("  ✅  Result: Valid/Invalid emails checked");
    }

    #[test]
    fn test_e164_phone_validation() {
        dbg!("\n✨ TEST: test_e164_phone_validation ✨");
        // Valid E.164 numbers
        assert!(is_valid_e164_phone("+14155552671"));      // US
        assert!(is_valid_e164_phone("+447911123456"));     // UK
        assert!(is_valid_e164_phone("+81312345678"));      // Japan
        assert!(is_valid_e164_phone("+15555551234"));      // US (10 digits)

        // Invalid numbers
        assert!(!is_valid_e164_phone("4155552671"));       // Missing '+'
        assert!(!is_valid_e164_phone("+1"));               // Too short
        assert!(!is_valid_e164_phone("+123456789012345678")); // Too long
        assert!(!is_valid_e164_phone("+1415abc2671"));     // Contains letters
        assert!(!is_valid_e164_phone("(415) 555-2671"));   // Wrong format
        dbg!("  ✅  Result: Phone numbers validated correctly");
    }

    #[test]
    fn test_clean_phone_number() {
        dbg!("\n✨ TEST: test_clean_phone_number ✨");
        // US numbers without country code (10 digits) - should add +1
        assert_eq!(clean_phone_number("4155552671").unwrap(), "+14155552671");
        assert_eq!(clean_phone_number("(415) 555-2671").unwrap(), "+14155552671");
        assert_eq!(clean_phone_number("415-555-2671").unwrap(), "+14155552671");
        assert_eq!(clean_phone_number("555.555.5555").unwrap(), "+15555555555");

        // US numbers with country code (11 digits starting with 1)
        assert_eq!(clean_phone_number("14155552671").unwrap(), "+14155552671");
        assert_eq!(clean_phone_number("1-415-555-2671").unwrap(), "+14155552671");
        assert_eq!(clean_phone_number("1 (415) 555-2671").unwrap(), "+14155552671");

        // International numbers with + prefix
        assert_eq!(clean_phone_number("+44 7911 123456").unwrap(), "+447911123456");
        assert_eq!(clean_phone_number("+81-3-1234-5678").unwrap(), "+81312345678");
        assert_eq!(clean_phone_number("+33 1 23 45 67 89").unwrap(), "+33123456789");

        // International numbers without + but with country code
        assert_eq!(clean_phone_number("447911123456").unwrap(), "+447911123456");
        assert_eq!(clean_phone_number("81312345678").unwrap(), "+81312345678");

        // Edge cases - should fail
        assert!(clean_phone_number("").is_err());           // Empty
        assert!(clean_phone_number("abc").is_err());        // No digits
        assert!(clean_phone_number("123").is_err());        // Too short
        assert!(clean_phone_number("12345678901234567890").is_err()); // Too long

        // Special characters should be stripped
        assert_eq!(clean_phone_number("(555) 555-5555").unwrap(), "+15555555555");
        assert_eq!(clean_phone_number("+1 (555) 555-5555").unwrap(), "+15555555555");

        // Numbers with extra digits are treated as international (assumed to have country code)
        assert_eq!(clean_phone_number("5555555555123").unwrap(), "+5555555555123");
        dbg!("  ✅  Result: Phone cleaning logic works as expected");
    }
}