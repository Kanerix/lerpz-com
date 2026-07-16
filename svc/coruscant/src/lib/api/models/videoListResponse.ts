// @ts-nocheck
import type { VideoItem } from './videoItem';

export interface VideoListResponse {
  /** The page of videos, newest first. */
  items: VideoItem[];
  /**
     * Cursor to pass as `cursor` for the next page, or `null` when the end has
     * been reached.
     * @nullable
     */
  next_cursor?: string | null;
}
