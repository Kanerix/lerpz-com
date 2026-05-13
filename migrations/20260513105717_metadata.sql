-- Storage provider for AI-generated assets.
CREATE TYPE storage_provider AS ENUM ('s3', 'abs');

-- Chat metadata.
-- Stores generation and optional analysis information for AI chat outputs.
CREATE TABLE chat_metadata (
    id          UUID        PRIMARY KEY DEFAULT uuidv7(),
    -- generation
    prompt      TEXT        NOT NULL,
    model       VARCHAR(255) NOT NULL,
    -- analysis (optional)
    title       VARCHAR(500),
    tags        TEXT[],
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON chat_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_chat_metadata_model ON chat_metadata(model);

-- Image metadata.
-- Stores generation, optional analysis, storage location, and dimensions for
-- AI-generated images.
CREATE TABLE image_metadata (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- generation
    prompt              TEXT            NOT NULL,
    model               VARCHAR(255)    NOT NULL,
    -- analysis (optional)
    title               VARCHAR(500),
    tags                TEXT[],
    -- storage
    storage_provider    storage_provider NOT NULL,
    storage_bucket      TEXT            NOT NULL,
    storage_key         TEXT            NOT NULL,
    -- dimensions
    width               INTEGER         NOT NULL CHECK (width > 0),
    height              INTEGER         NOT NULL CHECK (height > 0),
    created_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON image_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_image_metadata_model ON image_metadata(model);
CREATE INDEX idx_image_metadata_storage ON image_metadata(storage_provider, storage_bucket, storage_key);

-- Video metadata.
-- Stores generation, optional analysis, storage location, dimensions, and
-- duration for AI-generated videos.
CREATE TABLE video_metadata (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- generation
    prompt              TEXT            NOT NULL,
    model               VARCHAR(255)    NOT NULL,
    -- analysis (optional)
    title               VARCHAR(500),
    tags                TEXT[],
    -- storage
    storage_provider    storage_provider NOT NULL,
    storage_bucket      TEXT            NOT NULL,
    storage_key         TEXT            NOT NULL,
    -- dimensions and duration
    width               INTEGER         NOT NULL CHECK (width > 0),
    height              INTEGER         NOT NULL CHECK (height > 0),
    duration            INTEGER         NOT NULL CHECK (duration > 0),
    created_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON video_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_video_metadata_model ON video_metadata(model);
CREATE INDEX idx_video_metadata_storage ON video_metadata(storage_provider, storage_bucket, storage_key);

-- Audio metadata.
-- Stores generation, optional analysis, storage location, and duration for
-- AI-generated audio.
CREATE TABLE audio_metadata (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- generation
    prompt              TEXT            NOT NULL,
    model               VARCHAR(255)    NOT NULL,
    -- analysis (optional)
    title               VARCHAR(500),
    tags                TEXT[],
    -- storage
    storage_provider    storage_provider NOT NULL,
    storage_bucket      TEXT            NOT NULL,
    storage_key         TEXT            NOT NULL,
    -- duration
    duration            INTEGER         NOT NULL CHECK (duration > 0),
    created_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON audio_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_audio_metadata_model ON audio_metadata(model);
CREATE INDEX idx_audio_metadata_storage ON audio_metadata(storage_provider, storage_bucket, storage_key);
