-- Fix model_limits_enabled for existing tokens
-- Set to 1 for tokens that have model_limits_allowed configured

UPDATE api_tokens
SET model_limits_enabled = 1
WHERE model_limits_allowed IS NOT NULL
  AND model_limits_allowed != ''
  AND model_limits_allowed != '[]';
