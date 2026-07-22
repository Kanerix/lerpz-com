-- Video generation jobs
--
-- A video render is a long-running provider operation. Rather than holding an
-- SSE stream open for its full duration, the create endpoint records a job row,
-- spawns a background task that drives the render to completion, and returns
-- immediately. Clients poll this table (via the status endpoint) until the job
-- reaches a terminal state.
CREATE TABLE video_jobs (
    id          UUID          PRIMARY KEY DEFAULT uuidv7(),
    -- ownership (Azure object id of the requesting user)
    oid         TEXT          NOT NULL,
    -- lifecycle
    status      TEXT          NOT NULL DEFAULT 'in_progress'
                    CHECK (status IN ('in_progress', 'completed', 'failed')),
    -- human-readable failure reason, set when status = 'failed'
    error       TEXT,
    -- the persisted video, set when status = 'completed'
    video_id    UUID          REFERENCES video_metadata(id) ON DELETE SET NULL,
    -- other
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON video_jobs
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_video_jobs_oid ON video_jobs(oid);
