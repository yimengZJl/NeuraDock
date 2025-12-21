mod health_check;
mod task_manager;
mod task_spawner;
mod types;

use neuradock_domain::shared::AccountId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use types::TaskMetadata;

pub struct AutoCheckInScheduler {
    /// Active tasks mapped by account ID
    /// Using Mutex to allow modification from multiple contexts
    tasks: Arc<Mutex<HashMap<AccountId, JoinHandle<()>>>>,
    /// Task metadata for health monitoring
    task_metadata: Arc<Mutex<HashMap<AccountId, TaskMetadata>>>,
    /// Health check task handle
    health_check_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AutoCheckInScheduler {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            task_metadata: Arc::new(Mutex::new(HashMap::new())),
            health_check_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("✅ Auto check-in scheduler started (using tokio timer)");

        // Start health check task
        self.start_health_check_task().await;

        Ok(())
    }
}
