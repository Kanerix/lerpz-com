// @ts-nocheck

/**
 * Provider/runtime settings for a model, stored as a JSON object.
 *
 * The object is open-ended; only the keys the backend understands are
 * documented here.
 */
export interface ModelSettings {
  /**
     * Whether the model produces reasoning / chain-of-thought output that is
     * streamed via the `reasoning` SSE event and stored alongside replies.
     */
  reasoning?: boolean;
}
