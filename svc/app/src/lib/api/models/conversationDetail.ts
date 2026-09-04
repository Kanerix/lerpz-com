// @ts-nocheck
import type { ConversationMessage } from './conversationMessage';

/**
 * A conversation together with all its messages.
 */
export interface ConversationDetail {
  /** Whether the conversation has been archived by the user. */
  archived: boolean;
  /** Timestamp when the conversation was created. */
  created_at: string;
  /** Unique conversation identifier. */
  id: string;
  /** All messages in chronological order. */
  messages: ConversationMessage[];
  /** AI model used for this conversation. */
  model: string;
  /**
     * Conversation title.
     * @nullable
     */
  title?: string | null;
  /** Timestamp of the last update. */
  updated_at: string;
}
