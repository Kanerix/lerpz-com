// @ts-nocheck

export interface VideoItem {
  /** When the video was created. */
  created_at: string;
  /** Video duration in seconds. */
  duration: number;
  /** Container format (e.g. `mp4`, `webm`). */
  format: string;
  /** Video height in pixels. */
  height: number;
  /** Unique video ID. */
  id: string;
  /** Model that generated the video. */
  model: string;
  /** Prompt the video was generated from. */
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
  /** Video width in pixels. */
  width: number;
}
