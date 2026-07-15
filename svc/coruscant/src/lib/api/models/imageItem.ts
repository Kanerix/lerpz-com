// @ts-nocheck

export interface ImageItem {
  /** When the image was created. */
  created_at: string;
  /** Container format (e.g. `png`, `jpeg`). */
  format: string;
  /** Image height in pixels. */
  height: number;
  /** Unique image ID. */
  id: string;
  /** Model that generated the image. */
  model: string;
  /** Prompt the image was generated from. */
  prompt: string;
  /** Optional AI-generated tags. */
  tags: string[];
  /**
     * Optional AI-generated title.
     * @nullable
     */
  title?: string | null;
  /**
     * Publicly accessible URL served directly from the storage bucket, which
     * acts as a CDN.
     */
  url: string;
  /** Image width in pixels. */
  width: number;
}
