import type { ProblemSchema } from "$lib/api/models";

/**
 * Narrow an unknown thrown value to an RFC 9457 {@link ProblemSchema}.
 *
 * The backend sends `application/problem+json` bodies with (at minimum)
 * string `type`, `title`, and `detail` fields, so those are used as the
 * discriminator. Anything else (a network `Error`, a parsing failure, an
 * arbitrary thrown value) is treated as a non-problem error.
 */
export function isProblemSchema(value: unknown): value is ProblemSchema {
    if (typeof value !== "object" || value === null) return false;
    const candidate = value as Record<string, unknown>;
    return (
        typeof candidate.type === "string" &&
        typeof candidate.title === "string" &&
        typeof candidate.detail === "string"
    );
}

/** A human-readable message for a non-problem error value. */
export function getErrorMessage(error: unknown): string {
    if (error instanceof Error) return error.message;
    if (typeof error === "string") return error;
    return "An unexpected error occurred.";
}

/**
 * A full, console-style traceback for a non-problem error value.
 *
 * Prefers a real stack trace when available, then falls back to the message
 * or a JSON dump so the developer always has something to inspect.
 */
export function getErrorTraceback(error: unknown): string {
    if (error instanceof Error) {
        return error.stack ?? `${error.name}: ${error.message}`;
    }
    if (typeof error === "string") return error;
    try {
        return JSON.stringify(error, null, 2);
    } catch {
        return String(error);
    }
}

/**
 * Best-effort recovery of a rich error value from an arbitrary thrown value,
 * ready to hand to the {@link import('./ErrorDialog.svelte')} component.
 *
 * The generated API client's `customFetch` mutator throws an `Error` whose
 * message is `HTTP <status>: <body>`, where the body is usually an RFC 9457
 * `problem+json` payload. This unwraps that payload back into a
 * {@link ProblemSchema} so the dialog can render its rich problem view;
 * anything that can't be parsed as a problem is returned untouched for the
 * generic fallback view.
 */
export function toProblemError(error: unknown): unknown {
    if (isProblemSchema(error)) return error;

    const message =
        error instanceof Error
            ? error.message
            : typeof error === "string"
              ? error
              : null;
    if (!message) return error;

    const start = message.indexOf("{");
    const end = message.lastIndexOf("}");
    if (start !== -1 && end > start) {
        try {
            const parsed: unknown = JSON.parse(message.slice(start, end + 1));
            if (isProblemSchema(parsed)) return parsed;
        } catch {
            // Not a problem payload – fall through and keep the original error.
        }
    }
    return error;
}
