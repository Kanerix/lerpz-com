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
                onMessage: (data) => {
                    const stripped = data.startsWith("data:")
                        ? data.slice(5).trimStart()
                        : data;
                    image = `data:image/png;base64,${stripped}`;
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
                    if (image !== null) onDone?.(image);
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
