// @ts-nocheck

/**
 * Body for analysing a caller-supplied ("bring your own") image.
 */
export interface AnalyzeUploadRequest {
  /**
     * The image to analyse, base64-encoded.
     *
     * Both raw base64 and a full `data:` URL (e.g.
     * `data:image/png;base64,...`) are accepted; the leading data-URL prefix
     * is stripped before decoding.
     */
  image: string;
}
