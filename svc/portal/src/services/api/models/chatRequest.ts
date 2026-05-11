// @ts-nocheck

export interface ChatRequest {
  /**
     * Optional model override (uses the configured default when omitted)
     * @nullable
     */
  model?: string | null;
  /** The user's first message / opening prompt */
  prompt: string;
  /**
     * Optional conversation title; auto-generated from the prompt when omitted
     * @nullable
     */
  title?: string | null;
}
