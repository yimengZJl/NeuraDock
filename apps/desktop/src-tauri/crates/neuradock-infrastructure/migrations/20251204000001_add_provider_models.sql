-- Provider supported models table
-- Stores the list of models supported by each provider (fetched from API)
CREATE TABLE IF NOT EXISTS provider_models (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id TEXT NOT NULL,
    models TEXT NOT NULL,  -- JSON array of model names
    fetched_at TEXT NOT NULL,
    UNIQUE(provider_id)
);

CREATE INDEX IF NOT EXISTS idx_provider_models_provider ON provider_models(provider_id);
