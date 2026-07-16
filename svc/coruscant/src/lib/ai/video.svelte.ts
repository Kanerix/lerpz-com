import { createSseConnection } from "$lib/http/sse.js";
import { getCreateVideoUrl } from "$lib/api/videos/videos.js";
import type { VideoRequest } from "$lib/api/models/index.js";

export type UseVideoOptions = {
    model?: string;
    onDone?: (video: string) => void;
    onError?: (error: string) => void;
};

export type StartVideoOptions = {
    /** Model override for this request. */
    model?: string | null;
    /** Aspect ratio (e.g. "16:9"). Omit for the model default. */
    aspectRatio?: string | null;
    /** Clip length in seconds. Omit (or pass `null`) for the default. */
    duration?: number | null;
};

/**
 * Video frames arrive from the backend as a JSON object. A completed clip may
 * be delivered either as a ready-to-play URL (`url`) or as base64-encoded bytes
 * (`b64`) alongside the container format (e.g. `mp4`). Either way we resolve it
 * to a value assignable to a `<video src>`.
 */
function parseVideoFrame(data: string): string | null {
    try {
        const parsed = JSON.parse(data);
        if (parsed && typeof parsed.url === "string") {
            return parsed.url;
        }
        if (parsed && typeof parsed.b64 === "string") {
            const format =
                typeof parsed.format === "string" ? parsed.format : "mp4";
            return `data:video/${format};base64,${parsed.b64}`;
        }
    } catch {
        // Malformed payload – ignore this frame.
    }
    return null;
}

/**
 * Unwraps the message from an `error` event payload. The backend normalises
 * upstream provider errors into a single JSON-encoded string, so we just unwrap
 * it and fall back to a generic message.
 */
function parseErrorMessage(data: string): string {
    try {
        const parsed = JSON.parse(data);
        if (typeof parsed === "string") return parsed;
    } catch {
        // Fall through to the generic message.
    }
    return "An error occurred while generating the video.";
}

export function createVideo(options: UseVideoOptions = {}) {
    let video = $state<string | null>(null);
    let isLoading = $state(false);
    let isDone = $state(false);
    let error = $state<string | null>(null);

    let closeRef: (() => void) | null = null;

    const onDone = options.onDone;
    const onError = options.onError;

    $effect(() => {
        return () => closeRef?.();
    });

    function start(prompt: string, startOptions: StartVideoOptions = {}) {
        closeRef?.();
        closeRef = null;
        video = null;
        isLoading = true;
        isDone = false;
        error = null;

        const body = JSON.stringify({
            prompt,
            model: startOptions.model ?? options.model ?? null,
            aspect_ratio: startOptions.aspectRatio ?? null,
            duration: startOptions.duration ?? null,
        } satisfies VideoRequest);

        const { close } = createSseConnection(
            getCreateVideoUrl(),
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
                        // carry a video payload.
                        case "partial_video":
                        case "completed_video": {
                            const url = parseVideoFrame(data);
                            if (url !== null) video = url;
                            break;
                        }
                        // Metadata persisted successfully. Emitted after
                        // `completed_video`, so it must NOT be treated as video
                        // data; it simply confirms the save.
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
                        "An error occurred while streaming the video.";
                    onError?.(error ?? "");
                    closeRef = null;
                },
                onClose: () => {
                    isLoading = false;
                    isDone = true;
                    if (video !== null && error === null) onDone?.(video);
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
        video = null;
        isLoading = false;
        isDone = false;
        error = null;
    }

    return {
        get video() {
            return video;
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
