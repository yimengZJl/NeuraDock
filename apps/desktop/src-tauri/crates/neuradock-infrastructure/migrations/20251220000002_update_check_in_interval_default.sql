-- Update check_in_interval_hours to allow 0 (no limit)
-- Change all existing accounts from 23 hours to 0 (no limit) as the new default
UPDATE accounts SET check_in_interval_hours = 0;
