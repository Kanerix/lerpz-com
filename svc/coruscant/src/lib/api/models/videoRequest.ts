// @ts-nocheck

export interface VideoRequest {
  /**
     * Desired aspect ratio, e.g. `16:9` (landscape) or `9:16` (portrait).
     *
     * Defaults to portrait (`9:16`) if not provided or unrecognised.
     * @nullable
     */
  aspect_ratio?: string | null;
  /**
     * Clip length in seconds. Rounded down to the nearest supported value
     * (4, 8 or 12). Defaults to 4 seconds.
     * @minimum 0
     * @nullable
     */
  duration?: number | null;
  /**
     * What model to use.
     *
     * This will default to a predefined model if not provided.
     * @nullable
     */
  model?: string | null;
  /** Prompt that is sent to the model. */
  prompt: string;
}
