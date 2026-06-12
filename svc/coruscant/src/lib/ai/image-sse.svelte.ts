import { createSseConnection } from "$lib/http/sse.js";

export function createImageSse() {
    let image = $state<string | null>(null);
    let isLoading = $state(false);
    let isDone = $state(false);
    let error = $state<string | null>(null);

    let closeRef: (() => void) | null = null;

    $effect(() => {
        return () => closeRef?.();
    });

    function start(prompt: string) {
        closeRef?.();
        closeRef = null;
        image = null;
        isLoading = true;
        isDone = false;
        error = null;

        const { close } = createSseConnection(
            "/api/v1/images",
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ prompt }),
            },
            {
                onOpen: () => {
                    console.log("Image stream opened");
                },
                onMessage: (data) => {
                    const stripped = data.startsWith("data:")
                        ? data.slice(5).trimStart()
                        : data;
                    image = `data:image/png;base64,${stripped}`;
                },
                onError: (_err) => {
                    isLoading = false;
                    isDone = true;
                    error = "An error occurred while streaming the image.";
                    closeRef = null;
                },
                onClose: () => {
                    isLoading = false;
                    isDone = true;
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
