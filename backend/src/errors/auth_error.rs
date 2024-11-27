use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")]
    UserExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Database error")]
    DbError(#[from] sqlx::Error),
    #[error("Error creating token")]
    TokenCreationError,
    #[error("Hash error")]
    HashError(BcryptError),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<BcryptError> for AuthError {
    fn from(err: BcryptError) -> Self {
        AuthError::HashError(err)
    }
}
