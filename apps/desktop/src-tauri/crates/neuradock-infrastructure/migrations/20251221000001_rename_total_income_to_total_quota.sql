-- Rename legacy total_income columns to total_quota for semantic clarity
-- (migration runner already wraps execution in a transaction)

-- balances table
ALTER TABLE balances RENAME COLUMN total_income TO total_quota;

-- balance_history table (per-record snapshot)
ALTER TABLE balance_history RENAME COLUMN total_income TO total_quota;
