-- Add model_limits_enabled field to api_tokens table
-- This field indicates whether the token has model restrictions

ALTER TABLE api_tokens ADD COLUMN model_limits_enabled INTEGER NOT NULL DEFAULT 0;
