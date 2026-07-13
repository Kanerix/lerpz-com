// @ts-nocheck

export interface MessageRequest {
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
