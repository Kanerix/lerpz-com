// @ts-nocheck

export interface ImageRequest {
  /**
     * The amount of images to generate.
   *
   * This will default to 1 image if not provided.
     * @minimum 0
     * @nullable
     */
  amount?: number | null;
  /**
     * What model to use.
     * @nullable
     */
  model?: string | null;
  /** Prompt that is sent to the model. */
  prompt: string;
}
