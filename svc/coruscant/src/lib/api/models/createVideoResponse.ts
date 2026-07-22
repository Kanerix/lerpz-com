// @ts-nocheck

export interface CreateVideoResponse {
  /**
     * Identifier of the created job. Poll `GET /videos/jobs/{id}` with this to
     * track progress.
     */
  id: string;
  /** Current lifecycle status. Always `in_progress` for a freshly created job. */
  status: string;
}
