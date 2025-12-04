use thiserror::Error;

/// Repository errors that can occur during data access operations.
/// These are translated from database-specific errors (SQLx, etc.) by the Infrastructure layer.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Duplicate key error: {0}")]
    DuplicateKey(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal repository error: {0}")]
    Internal(String),
}

/// Errors that can occur during challenge storage operations.
#[derive(Debug, Error)]
pub enum ChallengeStoreError {
    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Challenge not found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Errors that can occur during WebAuthn verification.
#[derive(Debug, Error)]
pub enum WebAuthnError {
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Challenge mismatch")]
    ChallengeMismatch,

    #[error("Origin mismatch")]
    OriginMismatch,

    #[error("Internal error: {0}")]
    Internal(String),
}
