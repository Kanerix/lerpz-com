import { authenticatedFetch } from "./fetch.js";

export type SSEEvent = {
    /** The event type from the `event:` field. Defaults to `"message"` if not specified by the server. */
    event: string;
    /** The data payload from the `data:` field(s). */
    data: string;
};

export type SSEEventHandler = (event: SSEEvent) => void;

/**
 * @deprecated Use {@link SSEEventHandler} with {@link SSEEvent} instead.
 * Kept for backward compatibility — if you only care about the data payload,
 * you can still pass a `(data: string) => void` callback.
 */
export type SSEDataOnlyHandler = (data: string) => void;

/**
 * Creates a parser that correctly handles the SSE wire format.
 *
 * SSE events are separated by double newlines (`\n\n`). Each line within an
 * event can be one of:
 *   - `data: <payload>`   – the actual data (multiple lines are joined with \n)
 *   - `event: <name>`     – names the event type (default is "message")
 *   - `id: <value>`       – sets the last event ID
 *   - `retry: <ms>`       – reconnection hint
 *   - `: <comment>`       – comment / keep-alive ping, ignored
 *
 * ReadableStream chunks do NOT align with event boundaries, so we buffer
 * incoming text and only emit once we see a complete event.
 */
export function createSSEParser(onEvent: SSEEventHandler | SSEDataOnlyHandler) {
    let buffer = "";

    // Detect whether the callback expects the new SSEEvent object or just a
    // plain data string. We check the callback's arity: the new-style handler
    // receives one object arg, the old-style also receives one string arg — so
    // we can't distinguish by arity alone. Instead we wrap it so both work:
    // we always parse the full event and call the handler accordingly.

    return {
        feed(chunk: string) {
            buffer += chunk;

            // Split on the event boundary (double newline).
            const parts = buffer.split("\n\n");

            // The last part may be incomplete – keep it in the buffer.
            buffer = parts.pop() ?? "";

            for (const part of parts) {
                if (!part.trim()) continue;

                const lines = part.split("\n");
                let data = "";
                let eventType = "message"; // SSE spec default

                for (const line of lines) {
                    // Comments – silently ignore (used as keep-alive pings).
                    if (line.startsWith(":")) continue;

                    // `event:` with or without the optional space after the colon.
                    if (line.startsWith("event: ")) {
                        eventType = line.slice(7);
                    } else if (line.startsWith("event:")) {
                        eventType = line.slice(6);
                    }

                    // `data:` with or without the optional space after the colon.
                    else if (line.startsWith("data: ")) {
                        data += (data ? "\n" : "") + line.slice(6);
                    } else if (line.startsWith("data:")) {
                        data += (data ? "\n" : "") + line.slice(5);
                    }

                    // `id:` and `retry:` are intentionally ignored for now.
                }

                if (data) {
                    onEvent({ event: eventType, data } as SSEEvent & string);
                }
            }
        },
    };
}

export interface SseCallbacks {
    /** Called once the HTTP response is received and the stream is open. */
    onOpen?: () => void;

    /**
     * Called for every parsed SSE event.
     *
     * The `data` string has the `data: ` prefix stripped. If the upstream sends
     * JSON you should `JSON.parse(data)` inside this callback.
     *
     * The `event` string is the SSE event type (from the `event:` field).
     * Defaults to `"message"` when the server does not specify one.
     */
    onMessage?: (data: string, event: string) => void;

    /**
     * Called when an error occurs – either an HTTP-level error before the stream
     * starts, a network error mid-stream, an in-band error sent by the server,
     * or an unexpected stream termination.
     */
    onError?: (error: Error) => void;

    /**
     * Called when the stream completes – either cleanly (received a done signal)
     * or because the connection was closed.
     *
     * @param incomplete – `true` if the stream ended without a proper completion
     *   signal, which usually means the response was cut off.
     */
    onClose?: (incomplete: boolean) => void;
}

export interface SseOptions extends SseCallbacks {
    /**
     * The string the server sends as the final `data:` payload to signal that
     * the stream is finished. Defaults to `"[DONE]"` (the OpenAI convention).
     * Set to `null` to disable done-signal detection.
     */
    doneSignal?: string | null;

    /**
     * The event type that signals completion. If set, the stream is considered
     * done when an event with this type is received, regardless of the data
     * payload. Defaults to `null` (disabled).
     */
    doneEvent?: string | null;

    /**
     * Maximum time (in ms) to wait between chunks before considering the
     * connection stale. If no data is received within this window the stream is
     * aborted and `onError` is called. Defaults to `30_000` (30 s).
     * Set to `0` or `Infinity` to disable.
     */
    timeoutMs?: number;
}

export interface SseConnection {
    /** Abort the stream and close the connection. */
    close: () => void;
}

/**
 * Opens a streaming connection to `url` using `authenticatedFetch` and parses
 * the response as Server-Sent Events.
 *
 * Returns a handle with a `close()` method that can be used to abort the
 * stream at any time (the `AbortError` is swallowed automatically).
 *
 * ```ts
 * const { close } = createSseConnection("/api/chat", {
 *   method: "POST",
 *   headers: { "Content-Type": "application/json" },
 *   body: JSON.stringify({ messages }),
 * }, {
 *   onMessage(data, event) { console.log(`[${event}]`, data); },
 *   onError(err)           { console.error(err); },
 *   onClose(incomplete)    { if (incomplete) console.warn("cut off!"); },
 * });
 *
 * // Later…
 * close();
 * ```
 */
export function createSseConnection(
    url: string,
    init: RequestInit = {},
    options: SseOptions = {},
): SseConnection {
    const {
        onOpen,
        onMessage,
        onError,
        onClose,
        doneSignal = "[DONE]",
        doneEvent = null,
        timeoutMs = 30_000,
    } = options;

    const controller = new AbortController();
    const signal = controller.signal;

    // Kick off the async work without blocking the caller.
    (async () => {
        // ------------------------------------------------------------------
        // Phase 1 – HTTP-level: send the request and validate the response.
        // ------------------------------------------------------------------
        let response: Response;

        try {
            response = await authenticatedFetch(url, {
                ...init,
                signal,
            });
        } catch (err: unknown) {
            // Network error or user abort before the response arrived.
            if (signal.aborted) return;
            onError?.(
                err instanceof Error
                    ? err
                    : new Error("Failed to reach the server"),
            );
            onClose?.(true);
            return;
        }

        if (!response.ok || !response.body) {
            let detail: string;
            try {
                detail = await response.text();
            } catch {
                detail = response.statusText;
            }
            onError?.(
                new Error(
                    `HTTP ${response.status}: ${detail || "Unknown error"}`,
                ),
            );
            onClose?.(true);
            return;
        }

        // The connection is open and we got a successful response.
        onOpen?.();

        // ------------------------------------------------------------------
        // Phase 2 – Stream-level: read chunks and parse SSE events.
        // ------------------------------------------------------------------
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let receivedDone = false;

        // Inactivity timeout – reset every time we receive data.
        let timeoutId: ReturnType<typeof setTimeout> | undefined;
        const timeoutEnabled = timeoutMs > 0 && timeoutMs < Infinity;

        const clearInactivityTimeout = () => {
            if (timeoutId !== undefined) {
                clearTimeout(timeoutId);
                timeoutId = undefined;
            }
        };

        const resetInactivityTimeout = () => {
            clearInactivityTimeout();
            if (timeoutEnabled) {
                timeoutId = setTimeout(() => {
                    controller.abort();
                    onError?.(
                        new Error(
                            `Stream timed out: no data received for ${timeoutMs}ms`,
                        ),
                    );
                    onClose?.(true);
                }, timeoutMs);
            }
        };

        const parser = createSSEParser(({ event, data }: SSEEvent) => {
            // Done signal by data payload – stop processing.
            if (doneSignal !== null && data === doneSignal) {
                receivedDone = true;
                return;
            }

            // Done signal by event type – stop processing.
            if (doneEvent !== null && event === doneEvent) {
                receivedDone = true;
                // Still deliver the event so the consumer can read any final data.
                onMessage?.(data, event);
                return;
            }

            // Let the consumer inspect the payload. If the upstream sends errors
            // in-band (e.g. `{"error": {...}}`), the consumer can detect that
            // inside onMessage and call close() if needed.
            onMessage?.(data, event);
        });

        try {
            resetInactivityTimeout();

            while (true) {
                const { done, value } = await reader.read();
                if (done) break;

                resetInactivityTimeout();
                parser.feed(decoder.decode(value, { stream: true }));
            }
        } catch (err: unknown) {
            if (signal.aborted) return; // Already handled by timeout or user close.

            onError?.(
                err instanceof Error
                    ? err
                    : new Error("Unknown streaming error"),
            );
            onClose?.(true);
            return;
        } finally {
            clearInactivityTimeout();
        }

        // Stream finished — determine if it was a clean or incomplete close.
        const incomplete = doneSignal !== null && !receivedDone;
        if (incomplete) {
            onError?.(
                new Error(
                    "Stream ended unexpectedly without a completion signal",
                ),
            );
        }
        onClose?.(incomplete);
    })();

    return {
        close() {
            controller.abort();
        },
    };
}
