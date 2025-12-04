use thiserror::Error;
use crate::domain::errors::{RepositoryError, ChallengeStoreError, WebAuthnError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    ChallengeStore(#[from] ChallengeStoreError),

    #[error(transparent)]
    WebAuthn(#[from] WebAuthnError),
}

pub type AppResult<T> = Result<T, AppError>;