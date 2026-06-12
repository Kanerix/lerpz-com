// @ts-nocheck

/**
 * A single message within a conversation.
 */
export interface ConversationMessage {
    /** Raw message text. */
    content: string;
    /** Timestamp when the message was created. */
    created_at: string;
    /** Unique message identifier. */
    id: string;
    /** Message author: `"user"` or `"assistant"`. */
    role: string;
}
