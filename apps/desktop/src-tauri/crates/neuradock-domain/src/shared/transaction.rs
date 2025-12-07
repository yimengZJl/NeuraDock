use async_trait::async_trait;
use std::fmt;

/// Abstract transaction context for Unit of Work pattern.
/// This allows domain/application layers to work with transactions
/// without depending on specific database implementations (sqlx).
#[async_trait]
pub trait TransactionContext: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Commit the transaction
    async fn commit(self: Box<Self>) -> Result<(), Self::Error>;

    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> Result<(), Self::Error>;
}

/// Error type for Unit of Work operations
#[derive(Debug)]
pub enum UnitOfWorkError {
    TransactionFailed(String),
    CommitFailed(String),
    RollbackFailed(String),
}

impl fmt::Display for UnitOfWorkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            Self::CommitFailed(msg) => write!(f, "Commit failed: {}", msg),
            Self::RollbackFailed(msg) => write!(f, "Rollback failed: {}", msg),
        }
    }
}

impl std::error::Error for UnitOfWorkError {}

/// Unit of Work trait for managing transactional boundaries
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    type Transaction: TransactionContext;

    /// Begin a new transaction
    async fn begin(&self) -> Result<Box<Self::Transaction>, UnitOfWorkError>;
}
