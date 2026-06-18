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
     * Reasoning level for reasoning-capable models (`none`, `minimal`, `low`,
     * `medium`, `high` or `xhigh`). Unknown values fall back to `low`. Uses the
     * model's default behaviour when omitted.
     * @nullable
     */
  reasoning?: string | null;
  /**
     * Optional conversation title; auto-generated from the prompt when omitted
     * @nullable
     */
  title?: string | null;
}
