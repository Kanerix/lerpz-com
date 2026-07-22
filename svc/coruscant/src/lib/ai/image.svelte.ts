import { createSseConnection } from "$lib/http/sse.js";
import { getCreateImageUrl } from "$lib/api/images/images.js";
import type { ImageRequest } from "$lib/api/models/index.js";

export type UseImageOptions = {
    model?: string;
    onDone?: (image: string) => void;
    onError?: (error: string) => void;
};

export type StartImageOptions = {
    /** Model override for this request. */
    model?: string | null;
    /** Number of images to generate. Omit (or pass `null`) for the default. */
    amount?: number | null;
};

/**
 * Image frames arrive from the backend as a JSON object carrying the base64
 * data and the format reported by the model (`png` | `jpeg` | `webp`). The
 * format string doubles as the MIME subtype, so we can build the data URL
 * directly without sniffing the bytes.
 */
function parseImageFrame(data: string): string | null {
    try {
        const parsed = JSON.parse(data);
        if (parsed && typeof parsed.b64 === "string") {
            const format =
                typeof parsed.format === "string" ? parsed.format : "png";
            return `data:image/${format};base64,${parsed.b64}`;
        }
    } catch {
        // Malformed payload – ignore this frame.
    }
    return null;
}

/**
 * Unwraps the message from an `error` event payload.
 */
function parseErrorMessage(data: string): string {
    try {
        const parsed = JSON.parse(data);
        if (typeof parsed === "string") return parsed;
    } catch {
        // Fall through to the generic message.
    }
    return "An error occurred while generating the image.";
}

export function createImage(options: UseImageOptions = {}) {
    let image = $state<string | null>(null);
    let isLoading = $state(false);
    let isDone = $state(false);
    let error = $state<string | null>(null);

    let closeRef: (() => void) | null = null;

    let onDone = options.onDone;
    let onError = options.onError;

    $effect(() => {
        return () => closeRef?.();
    });

    function start(prompt: string, startOptions: StartImageOptions = {}) {
        closeRef?.();
        closeRef = null;
        image = null;
        isLoading = true;
        isDone = false;
        error = null;

        const body = JSON.stringify({
            prompt,
            model: startOptions.model ?? options.model ?? null,
            amount: startOptions.amount ?? null,
        } satisfies ImageRequest);

        const { close } = createSseConnection(
            getCreateImageUrl(),
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body,
            },
            {
                doneSignal: null,
                onMessage: (data, event) => {
                    switch (event) {
                        // Progressive preview frames and the final render both
                        // carry a { b64, format } image payload.
                        case "partial_image":
                        case "completed_image": {
                            const url = parseImageFrame(data);
                            if (url !== null) image = url;
                            break;
                        }
                        // Metadata persisted successfully. This is emitted after
                        // `completed_image`, so we must NOT treat its payload as
                        // image data; it simply confirms the save.
                        case "saved":
                            break;
                        // In-band error from the server.
                        case "error": {
                            error = parseErrorMessage(data);
                            isLoading = false;
                            isDone = true;
                            onError?.(error);
                            closeRef?.();
                            closeRef = null;
                            break;
                        }
                    }
                },
                onError: (err) => {
                    isLoading = false;
                    isDone = true;
                    error =
                        err.message ||
                        "An error occurred while streaming the image.";
                    onError?.(error ?? "");
                    closeRef = null;
                },
                onClose: () => {
                    isLoading = false;
                    isDone = true;
                    if (image !== null && error === null) onDone?.(image);
                    closeRef = null;
                },
            },
        );

        closeRef = close;
    }

    function stop() {
        closeRef?.();
        closeRef = null;
        isLoading = false;
        isDone = true;
    }

    function reset() {
        closeRef?.();
        closeRef = null;
        image = null;
        isLoading = false;
        isDone = false;
        error = null;
    }

    return {
        get image() {
            return image;
        },
        get isLoading() {
            return isLoading;
        },
        get isDone() {
            return isDone;
        },
        get error() {
            return error;
        },
        start,
        stop,
        reset,
    };
}
