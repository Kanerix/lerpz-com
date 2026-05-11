// @ts-nocheck

export interface Model {
  /** Provider family identifier (e.g. `openai`, `google-ai`) */
  family: string;
  /** Human-readable model name (e.g. `gpt-image-1`) */
  name: string;
  /** Namespaced slug used when making inference requests (e.g. `@azure/gpt-image-1`) */
  slug: string;
}
