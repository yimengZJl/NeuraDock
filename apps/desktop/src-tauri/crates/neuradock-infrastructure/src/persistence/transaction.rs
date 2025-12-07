use async_trait::async_trait;
use neuradock_domain::shared::transaction::{TransactionContext, UnitOfWork, UnitOfWorkError};
use sqlx::{Pool, Sqlite, Transaction as SqlxTransaction};
use std::sync::Arc;

/// Sqlite implementation of TransactionContext
pub struct SqliteTransactionContext<'a> {
    tx: Option<SqlxTransaction<'a, Sqlite>>,
}

impl<'a> SqliteTransactionContext<'a> {
    pub fn new(tx: SqlxTransaction<'a, Sqlite>) -> Self {
        Self { tx: Some(tx) }
    }

    /// Get mutable reference to the underlying transaction
    /// This is used by repositories to execute queries within the transaction
    pub fn inner_mut(&mut self) -> &mut SqlxTransaction<'a, Sqlite> {
        self.tx.as_mut().expect("Transaction already consumed")
    }
}

#[async_trait]
impl<'a> TransactionContext for SqliteTransactionContext<'a> {
    type Error = sqlx::Error;

    async fn commit(mut self: Box<Self>) -> Result<(), Self::Error> {
        if let Some(tx) = self.tx.take() {
            tx.commit().await
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn rollback(mut self: Box<Self>) -> Result<(), Self::Error> {
        if let Some(tx) = self.tx.take() {
            tx.rollback().await
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }
}

/// Sqlite implementation of Unit of Work
pub struct SqliteUnitOfWork {
    pool: Arc<Pool<Sqlite>>,
}

impl SqliteUnitOfWork {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for SqliteUnitOfWork {
    type Transaction = SqliteTransactionContext<'static>;

    async fn begin(&self) -> Result<Box<Self::Transaction>, UnitOfWorkError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UnitOfWorkError::TransactionFailed(e.to_string()))?;

        // SAFETY: We're converting the transaction to 'static lifetime
        // This is safe because the transaction will be committed or rolled back
        // before the pool is dropped
        let static_tx: SqlxTransaction<'static, Sqlite> = unsafe { std::mem::transmute(tx) };

        Ok(Box::new(SqliteTransactionContext::new(static_tx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn test_unit_of_work_commit() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let uow = SqliteUnitOfWork::new(Arc::new(pool));

        let tx = uow.begin().await.unwrap();
        tx.commit().await.unwrap();
    }

    #[sqlx::test]
    async fn test_unit_of_work_rollback() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let uow = SqliteUnitOfWork::new(Arc::new(pool));

        let tx = uow.begin().await.unwrap();
        tx.rollback().await.unwrap();
    }
}
