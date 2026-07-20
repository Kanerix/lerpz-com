-- Theme preference
CREATE TYPE theme_pref AS ENUM ('light', 'dark', 'system');

-- User settings table
--
-- One row per user, keyed by the identity provider's subject/user id. Backs the
-- account settings surface (appearance and notification preferences).
CREATE TABLE user_settings (
    -- ownership
    user_id                 VARCHAR(255)    PRIMARY KEY,
    -- appearance
    theme                   theme_pref      NOT NULL DEFAULT 'system',
    -- notifications
    notify_product_updates  BOOLEAN         NOT NULL DEFAULT TRUE,
    notify_activity_digest  BOOLEAN         NOT NULL DEFAULT FALSE,
    notify_security_alerts  BOOLEAN         NOT NULL DEFAULT TRUE,
    -- other
    created_at              TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON user_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
