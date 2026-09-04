/**
 * Format an ISO timestamp as a short, locale-aware date and time (e.g.
 * `Jan 5, 2025, 14:30`). Returns an em dash for missing or unparseable values.
 */
export function formatDate(value: string | null | undefined): string {
    if (!value) return "—";
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return "—";
    return date.toLocaleString(undefined, {
        year: "numeric",
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
    });
}

/** Format a duration in seconds as `m:ss` (e.g. `1:05`). */
export function formatDuration(seconds: number): string {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
}
