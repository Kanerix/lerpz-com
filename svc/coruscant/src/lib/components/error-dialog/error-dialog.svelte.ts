import { toProblemError } from "./problem.js";

/**
 * Detects an abort/cancellation so a superseded request (e.g. an in-flight
 * query cancelled by navigation) never surfaces as a user-facing error.
 */
function isAbortError(value: unknown): boolean {
    return value instanceof Error && value.name === "AbortError";
}

function createErrorDialogStore() {
    let open = $state(false);
    let error = $state<unknown>(null);

    return {
        get open() {
            return open;
        },
        set open(value: boolean) {
            open = value;
            // Drop the payload on close so a stale error can't flash back the
            // next time the dialog opens.
            if (!value) error = null;
        },
        get error() {
            return error;
        },
        /**
         * Surface a thrown value through the dialog. The value is normalised to
         * an RFC 9457 problem where possible so the rich problem view is used;
         * nullish values and aborted/cancelled requests are ignored.
         */
        show(value: unknown) {
            if (value == null || isAbortError(value)) return;
            error = toProblemError(value);
            open = true;
        },
        dismiss() {
            open = false;
            error = null;
        },
    };
}

/**
 * Application-wide error dialog state. A single `ErrorDialog` bound to this
 * store is mounted once in the root layout, so any code can raise an error
 * through {@link showError} (or the TanStack Query cache) without wiring up its
 * own dialog.
 */
export const errorDialog = createErrorDialogStore();

/**
 * Surface any thrown value through the global error dialog. This is the single
 * entry point for "the initial response contained an error" — use it in the
 * `catch` block of any direct API call so error handling stays uniform.
 */
export function showError(value: unknown): void {
    errorDialog.show(value);
}
