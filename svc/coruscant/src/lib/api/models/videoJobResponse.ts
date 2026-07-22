// @ts-nocheck
import type { JobVideo } from './jobVideo';

export interface VideoJobResponse {
  /**
     * Human-readable failure reason. Present only when `status` is `failed`.
     * @nullable
     */
  error?: string | null;
  /** The job ID. */
  id: string;
  /** Lifecycle status: `in_progress`, `completed`, or `failed`. */
  status: string;
  /** The generated video. Present only when `status` is `completed`. */
  video?: JobVideo;
}
