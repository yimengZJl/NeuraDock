-- Create proxy_settings table to store proxy configuration
CREATE TABLE IF NOT EXISTS proxy_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Singleton table, only one row allowed
    enabled BOOLEAN NOT NULL DEFAULT 0,
    proxy_type TEXT NOT NULL DEFAULT 'http' CHECK(proxy_type IN ('http', 'socks5')),
    host TEXT NOT NULL DEFAULT '',
    port INTEGER NOT NULL DEFAULT 0 CHECK(port >= 0 AND port <= 65535),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default disabled proxy configuration
INSERT INTO proxy_settings (id, enabled, proxy_type, host, port)
VALUES (1, 0, 'http', '', 0);
