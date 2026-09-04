// @ts-nocheck

/**
 * Title and tags produced by the vision model for an image.
 *
 * Shared by both the stored-image analysis endpoint and the
 * "bring your own image" upload endpoint.
 */
export interface ImageAnalysisResponse {
  /** AI-generated tags describing the image. */
  tags: string[];
  /** AI-generated title describing the image. */
  title: string;
}
