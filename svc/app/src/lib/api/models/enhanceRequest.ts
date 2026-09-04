// @ts-nocheck

export interface EnhanceRequest {
  /**
     * Optional model override.
     * @nullable
     */
  model?: string | null;
  /** The raw prompt to enhance. */
  prompt: string;
}
