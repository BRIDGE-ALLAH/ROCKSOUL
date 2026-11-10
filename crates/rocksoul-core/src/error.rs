use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("The requested project or mission was not found.")]
    NotFound,

    #[error("The selected project directory is not available: {0}")]
    ProjectDirectory(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Database migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Background inventory task failed: {0}")]
    Task(String),
}
