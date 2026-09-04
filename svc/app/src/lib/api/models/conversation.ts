// @ts-nocheck

export interface Conversation {
  /** Whether the conversation has been archived by the user */
  archived: boolean;
  /**
     * Timestamp of when the conversation was created
     * @nullable
     */
  created_at?: string | null;
  /** Unique conversation identifier */
  id: string;
  /** AI model used for this conversation */
  model: string;
  /**
     * Conversation title (auto-generated from the first prompt if not provided)
     * @nullable
     */
  title?: string | null;
  /**
     * Timestamp of the last message in the conversation
     * @nullable
     */
  updated_at?: string | null;
}
