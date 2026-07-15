// @ts-nocheck
import type { ImageItem } from './imageItem';

export interface ImageListResponse {
  /** The page of images, newest first. */
  items: ImageItem[];
  /**
     * Cursor to pass as `cursor` for the next page, or `null` when the end has
     * been reached.
     * @nullable
     */
  next_cursor?: string | null;
}
