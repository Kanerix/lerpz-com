-- Storage provider
CREATE TYPE storage_provider AS ENUM ('s3', 'abs');

-- Image metadata table
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
    -- image info
    format              TEXT            NOT NULL,
    width               INTEGER         NOT NULL CHECK (width > 0),
    height              INTEGER         NOT NULL CHECK (height > 0),
    -- other
    created_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON image_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_image_metadata_model ON image_metadata(model);
CREATE INDEX idx_image_metadata_storage ON image_metadata(storage_provider, storage_bucket, storage_key);

-- Video metadata table
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
    -- video info
    format              TEXT            NOT NULL,
    width               INTEGER         NOT NULL CHECK (width > 0),
    height              INTEGER         NOT NULL CHECK (height > 0),
    duration            INTEGER         NOT NULL CHECK (duration > 0),
    -- other
    created_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON video_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_video_metadata_model ON video_metadata(model);
CREATE INDEX idx_video_metadata_storage ON video_metadata(storage_provider, storage_bucket, storage_key);

-- Audio metadata table
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
    -- audio info
    format              TEXT            NOT NULL,
    duration            INTEGER         NOT NULL CHECK (duration > 0),
    -- other
    created_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON audio_metadata
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_audio_metadata_model ON audio_metadata(model);
CREATE INDEX idx_audio_metadata_storage ON audio_metadata(storage_provider, storage_bucket, storage_key);
