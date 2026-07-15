-- Model family
CREATE TYPE model_family AS ENUM ('openai', 'anthropic', 'google');

-- Models table
CREATE TABLE models (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- identity
    display_name        VARCHAR(255)    NOT NULL,
    description         TEXT,
    family              model_family    NOT NULL,
    -- portkey routing
    deployment_name     VARCHAR(255)    NOT NULL,
    provider            VARCHAR(255)    NOT NULL,
    -- capabilities
    modalities          TEXT[]          NOT NULL DEFAULT '{}',
    -- configuration
    settings            JSONB           NOT NULL DEFAULT '{}'::jsonb,
    -- other
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (provider, deployment_name)
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON models
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_models_family ON models(family);
CREATE INDEX idx_models_provider ON models(provider);
