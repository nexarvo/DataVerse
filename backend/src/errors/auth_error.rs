#[derive(Debug, thiserror::Error)]
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
    HashError(#[from] bcrypt::BcryptError),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Unauthorized")]
    Unauthorized
}