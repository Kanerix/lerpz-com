// @ts-nocheck

export type ListImagesParams = {
/**
 * Return images older than this ID (exclusive).
 *
 * Pass the `next_cursor` from the previous page to fetch the following
 * one; omit for the first page.
 */
cursor?: string;
/**
 * Maximum number of images to return. Defaults to 24, capped at 100.
 */
limit?: number;
};
