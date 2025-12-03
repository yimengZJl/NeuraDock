-- WAF cookies cache table
-- Stores WAF cookies per provider to avoid repeated browser automation
CREATE TABLE IF NOT EXISTS waf_cookies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id TEXT NOT NULL UNIQUE,
    cookies TEXT NOT NULL,  -- JSON object of cookies
    fetched_at TEXT NOT NULL,  -- ISO 8601 timestamp
    expires_at TEXT NOT NULL   -- ISO 8601 timestamp (fetched_at + 24 hours)
);

-- Index for quick lookup by provider
CREATE INDEX IF NOT EXISTS idx_waf_cookies_provider ON waf_cookies(provider_id);
