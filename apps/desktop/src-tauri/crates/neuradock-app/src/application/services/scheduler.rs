use chrono::Local;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::Duration;
use tracing::{error, info, instrument, warn};

use neuradock_domain::account::AccountRepository;
use neuradock_domain::check_in::Provider;
use neuradock_domain::shared::AccountId;

/// Task metadata for health monitoring
#[derive(Debug, Clone)]
struct TaskMetadata {
    account_id: AccountId,
    account_name: String,
    started_at: chrono::DateTime<chrono::Utc>,
    last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

/// Configuration for spawning a check-in task
struct CheckInTaskConfig {
    account_id: AccountId,
    account_name: String,
    hour: u8,
    minute: u8,
    provider: Provider,
    account_repo: Arc<dyn AccountRepository>,
    app_handle: tauri::AppHandle,
}

pub struct AutoCheckInScheduler {
    account_repo: Arc<dyn AccountRepository>,
    /// Active tasks mapped by account ID
    /// Using Mutex to allow modification from multiple contexts
    tasks: Arc<Mutex<HashMap<AccountId, JoinHandle<()>>>>,
    /// Task metadata for health monitoring
    task_metadata: Arc<Mutex<HashMap<AccountId, TaskMetadata>>>,
    /// Health check task handle
    health_check_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AutoCheckInScheduler {
    pub async fn new(
        account_repo: Arc<dyn AccountRepository>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            account_repo,
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

    /// Start health check background task to monitor scheduled tasks
    async fn start_health_check_task(&self) {
        let tasks = Arc::clone(&self.tasks);
        let metadata = Arc::clone(&self.task_metadata);

        let handle = tokio::spawn(async move {
            let mut check_interval = tokio::time::interval(Duration::from_secs(300)); // Check every 5 minutes

            loop {
                check_interval.tick().await;

                let tasks_lock = tasks.lock().await;
                let mut metadata_lock = metadata.lock().await;

                let mut dead_tasks = Vec::new();

                for (account_id, handle) in tasks_lock.iter() {
                    if handle.is_finished() {
                        warn!(
                            "🔴 Health Check: Task for account {} has terminated unexpectedly",
                            account_id.as_str()
                        );
                        dead_tasks.push(account_id.clone());
                    } else if let Some(meta) = metadata_lock.get(account_id) {
                        // Check if task hasn't executed for more than 25 hours (should execute daily)
                        if let Some(last_exec) = meta.last_execution {
                            let elapsed = chrono::Utc::now() - last_exec;
                            if elapsed > chrono::Duration::hours(25) {
                                warn!(
                                    "⚠️  Health Check: Task for '{}' hasn't executed in {} hours",
                                    meta.account_name,
                                    elapsed.num_hours()
                                );
                            }
                        }
                    }
                }

                // Remove metadata for dead tasks
                for account_id in dead_tasks {
                    metadata_lock.remove(&account_id);
                    error!(
                        "🔴 Health Check: Removed dead task metadata for account {}",
                        account_id.as_str()
                    );
                }

                drop(tasks_lock);
                drop(metadata_lock);
            }
        });

        let mut health_check = self.health_check_handle.lock().await;
        *health_check = Some(handle);

        info!("✅ Health check task started (checking every 5 minutes)");
    }

    /// Stop all scheduled tasks
    pub async fn stop_all_tasks(&self) {
        // Stop health check task first
        let mut health_check = self.health_check_handle.lock().await;
        if let Some(handle) = health_check.take() {
            handle.abort();
            info!("🛑 Health check task stopped");
        }
        drop(health_check);

        let mut tasks = self.tasks.lock().await;
        let mut metadata = self.task_metadata.lock().await;

        info!("🛑 Stopping {} scheduled tasks...", tasks.len());

        for (account_id, handle) in tasks.drain() {
            info!("  ⏹️  Stopping task for account: {}", account_id.as_str());
            handle.abort();
        }

        metadata.clear();

        info!("✅ All scheduled tasks stopped");
    }

    /// Get count of active tasks
    pub async fn active_task_count(&self) -> usize {
        self.tasks.lock().await.len()
    }

    #[instrument(skip(self, providers, account_repo, app_handle))]
    pub async fn reload_schedules(
        &self,
        providers: HashMap<String, Provider>,
        account_repo: Arc<dyn AccountRepository>,
        app_handle: tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Reloading auto check-in schedules");

        // 1. Stop all existing tasks first
        let existing_count = self.active_task_count().await;
        if existing_count > 0 {
            info!(
                "🔄 Stopping {} existing scheduled tasks before reload...",
                existing_count
            );
            self.stop_all_tasks().await;
        }

        let now = Local::now();
        info!(
            "📍 Current local time: {}",
            now.format("%Y-%m-%d %H:%M:%S %Z")
        );

        // 2. Get all accounts with auto check-in enabled
        let accounts = account_repo.find_all().await?;
        info!("Found {} total accounts", accounts.len());

        let mut scheduled_count = 0;
        for account in accounts {
            info!(
                "Account: {} - enabled: {}, auto_checkin: {}",
                account.name(),
                account.is_enabled(),
                account.auto_checkin_enabled()
            );

            if account.auto_checkin_enabled() && account.is_enabled() {
                let provider_id = account.provider_id().as_str();
                if let Some(provider) = providers.get(provider_id) {
                    self.spawn_check_in_task(CheckInTaskConfig {
                        account_id: account.id().clone(),
                        account_name: account.name().to_string(),
                        hour: account.auto_checkin_hour(),
                        minute: account.auto_checkin_minute(),
                        provider: provider.clone(),
                        account_repo: account_repo.clone(),
                        app_handle: app_handle.clone(),
                    })
                    .await;
                    scheduled_count += 1;
                } else {
                    info!(
                        "⚠️  Provider '{}' not found for account {}",
                        provider_id,
                        account.name()
                    );
                }
            }
        }

        info!("✅ Scheduled {} auto check-in jobs", scheduled_count);

        Ok(())
    }

    async fn spawn_check_in_task(&self, config: CheckInTaskConfig) {
        // Destructure config for easier use
        let CheckInTaskConfig {
            account_id,
            account_name,
            hour,
            minute,
            provider,
            account_repo,
            app_handle,
        } = config;

        info!(
            "➕ Spawning task for '{}' at {}:{:02}",
            account_name, hour, minute
        );

        // Clone account_id before moving it into the async closure
        let account_id_for_storage = account_id.clone();
        let account_name_clone = account_name.clone();

        // Clone task metadata for updating within the task
        let task_metadata = Arc::clone(&self.task_metadata);

        // Initialize metadata
        {
            let mut metadata = task_metadata.lock().await;
            metadata.insert(
                account_id_for_storage.clone(),
                TaskMetadata {
                    account_id: account_id_for_storage.clone(),
                    account_name: account_name_clone.clone(),
                    started_at: chrono::Utc::now(),
                    last_execution: None,
                },
            );
        }

        let handle = tokio::spawn(async move {
            loop {
                let now = Local::now();
                // Validate and clamp hour/minute to valid ranges to prevent panics
                let target_hour = (hour as u32).min(23);
                let target_minute = (minute as u32).min(59);

                // Warn if values were clamped
                if hour > 23 || minute > 59 {
                    error!(
                        "⚠️  Invalid schedule time for account '{}': {}:{} (clamped to {}:{})",
                        account_name, hour, minute, target_hour, target_minute
                    );
                }

                // Calculate next execution time with proper error handling
                let next_run = match now
                    .date_naive()
                    .and_hms_opt(target_hour, target_minute, 0)
                    .and_then(|dt| dt.and_local_timezone(now.timezone()).single())
                {
                    Some(mut next) => {
                        // If the target time has already passed today, schedule for tomorrow
                        if next <= now {
                            next += chrono::Duration::days(1);
                        }
                        next
                    }
                    None => {
                        error!(
                            "❌ Failed to calculate next run time for account '{}' with time {}:{}. Task will exit.",
                            account_name, target_hour, target_minute
                        );
                        break; // Exit the loop to stop this task
                    }
                };

                let duration_until_next =
                    (next_run - now).to_std().unwrap_or(Duration::from_secs(60));

                info!(
                    "Next run for '{}': {} (in {} seconds)",
                    account_name,
                    next_run.format("%Y-%m-%d %H:%M:%S"),
                    duration_until_next.as_secs()
                );

                // Sleep until next execution
                tokio::time::sleep(duration_until_next).await;

                // Execute check-in
                info!(
                    "⏰ [AUTO CHECK-IN] Executing for account: {} at {}",
                    account_name,
                    Local::now().format("%Y-%m-%d %H:%M:%S %Z")
                );

                // Update last execution time
                {
                    let mut metadata = task_metadata.lock().await;
                    if let Some(meta) = metadata.get_mut(&account_id) {
                        meta.last_execution = Some(chrono::Utc::now());
                    }
                }

                use crate::application::services::CheckInExecutor;
                match CheckInExecutor::new(account_repo.clone(), true) {
                    Ok(executor) => {
                        match executor
                            .execute_check_in(account_id.as_str(), &provider)
                            .await
                        {
                            Ok(result) => {
                                if result.success {
                                    info!(
                                        "✅ [AUTO CHECK-IN] Success for {}: {}",
                                        account_name, result.message
                                    );

                                    // Send notification
                                    use tauri_plugin_notification::NotificationExt;
                                    if let Err(e) = app_handle
                                        .notification()
                                        .builder()
                                        .title("Auto Check-in Success")
                                        .body(format!("{}: {}", account_name, result.message))
                                        .show()
                                    {
                                        error!(
                                            "❌ [AUTO CHECK-IN] Failed to send notification: {}",
                                            e
                                        );
                                    }
                                } else {
                                    error!(
                                        "❌ [AUTO CHECK-IN] Failed for {}: {}",
                                        account_name, result.message
                                    );
                                }
                            }
                            Err(e) => {
                                error!("❌ [AUTO CHECK-IN] Error for {}: {}", account_name, e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("❌ [AUTO CHECK-IN] Failed to create executor: {}", e);
                    }
                }
            }
        });

        // Store the task handle for later cleanup
        let mut tasks = self.tasks.lock().await;

        // If there was an old task for this account, abort it
        if let Some(old_handle) = tasks.insert(account_id_for_storage.clone(), handle) {
            warn!(
                "⚠️  Aborting old task for account: {}",
                account_id_for_storage.as_str()
            );
            old_handle.abort();
        }

        info!(
            "✅ Task registered for account: {}",
            account_id_for_storage.as_str()
        );
    }

    pub async fn shutdown(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🛑 Shutting down auto check-in scheduler");
        self.stop_all_tasks().await;
        Ok(())
    }
}
