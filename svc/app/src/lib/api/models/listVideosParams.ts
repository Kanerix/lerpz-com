// @ts-nocheck

export type ListVideosParams = {
/**
 * Return videos older than this ID (exclusive).
 *
 * Pass the `next_cursor` from the previous page to fetch the following
 * one; omit for the first page.
 */
cursor?: string;
/**
 * Maximum number of videos to return. Defaults to 24, capped at 100.
 */
limit?: number;
};
