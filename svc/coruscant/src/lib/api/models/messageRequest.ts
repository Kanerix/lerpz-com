// @ts-nocheck

export interface MessageRequest {
  /**
     * Optional model override. Switches the conversation to this model for
     * this and future messages. Uses the conversation's current model when
     * omitted.
     * @nullable
     */
  model?: string | null;
  /** The user's message text */
  prompt: string;
  /**
     * Reasoning level for reasoning-capable models (`none`, `minimal`, `low`,
     * `medium`, `high` or `xhigh`). Unknown values fall back to `low`. Uses
     * the model's default behaviour when omitted.
     * @nullable
     */
  reasoning?: string | null;
}
