-- ============================================================
-- Drop Unused Tables
-- ============================================================

-- Drop check_in_jobs table and its indexes
DROP INDEX IF EXISTS idx_jobs_account;
DROP INDEX IF EXISTS idx_jobs_status;
DROP INDEX IF EXISTS idx_jobs_scheduled;
DROP TABLE IF EXISTS check_in_jobs;

-- Drop notification_history table and its indexes
DROP INDEX IF EXISTS idx_notification_channel;
DROP TABLE IF EXISTS notification_history;
