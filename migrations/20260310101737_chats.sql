-- Conversations table
CREATE TABLE conversations (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- ownership
    user_id             VARCHAR(255)    NOT NULL,
    -- content
    title               VARCHAR(500),
    model               VARCHAR(255)    NOT NULL,
    -- state
    archived            BOOLEAN         NOT NULL DEFAULT FALSE,
    -- other
    created_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON conversations
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_conversations_user_id ON conversations(user_id);

-- Message role
CREATE TYPE message_role AS ENUM ('user', 'assistant');

-- Messages table
CREATE TABLE messages (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    -- relationship
    conversation_id     UUID            NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    -- content
    role                message_role    NOT NULL,
    content             TEXT            NOT NULL,
    reasoning           TEXT,
    -- generation
    model_family        VARCHAR(255),
    -- other
    created_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
    BEFORE UPDATE ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

CREATE INDEX idx_messages_conversation_id ON messages(conversation_id);
