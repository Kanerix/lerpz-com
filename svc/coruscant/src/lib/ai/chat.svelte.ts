import { createSseConnection, type SseProblemError } from "$lib/http/sse.js";
import {
    getCreateChatUrl,
    getSendChatMessageUrl,
} from "$lib/api/chats/chats.js";
import type {
    ChatRequest,
    ConversationMessage,
    MessageRequest,
} from "$lib/api/models/index.js";
import { isProblemSchema } from "$lib/components/error-dialog/problem.js";

export type ChatMessage = ConversationMessage;

/**
 * Turn an in-band `error` SSE payload into the richest value we can render.
 *
 * The server may send a JSON `application/problem+json` string; when it does we
 * surface the parsed {@link ProblemSchema} so the error dialog can show its rich
 * view. Otherwise the raw string is kept as-is.
 */
function toErrorValue(data: string): unknown {
    try {
        const parsed: unknown = JSON.parse(data);
        if (isProblemSchema(parsed)) return parsed;
    } catch {
        // Not JSON – fall through and keep the raw string.
    }
    return data;
}

let _nextTempId = 0;
function tempId(): string {
    return `__temp_${++_nextTempId}`;
}

export type UseChatOptions = {
    model?: string;
    title?: string;
    onSaved?: (convId: string) => void;
    onError?: (error: string) => void;
};

export type SendChatOptions = {
    model?: string | null;
    reasoning?: string | null;
};

export function createChat(options: UseChatOptions = {}) {
    let conversationId = $state<string | null>(null);
    let messages = $state<ConversationMessage[]>([]);
    let isLoading = $state(false);
    let isStreaming = $state(false);
    let isSaved = $state(false);
    let error = $state<string | null>(null);
    // The raw error value (a `ProblemSchema`, `Error`, or string) kept so the
    // error dialog can render it richly. `error` above stays a plain message
    // for compact text surfaces like the status bar.
    let errorValue = $state<unknown>(null);

    let closeRef: (() => void) | null = null;
    let assistantBuf = "";
    let reasoningBuf = "";
    let assistantMsgId = tempId();
    let conversationIdRef: string | null = null;

    // Callbacks via refs to avoid stale closures
    let onSaved = options.onSaved;
    let onError = options.onError;

    // Remember the options of the most recent send so `retry()` can replay it
    // with the same model/reasoning.
    let lastSendOptions: SendChatOptions = {};

    $effect(() => {
        return () => closeRef?.();
    });

    // Remove the failed exchange – the last user message and any partial
    // assistant reply streamed before the error. Used when the user retries or
    // sends a new message so a stale "not sent" bubble doesn't linger.
    function discardFailedExchange() {
        for (let i = messages.length - 1; i >= 0; i--) {
            if (messages[i]?.role === "user") {
                messages = messages.slice(0, i);
                return;
            }
        }
    }

    function send(prompt: string, sendOptions: SendChatOptions = {}) {
        closeRef?.();
        closeRef = null;
        lastSendOptions = sendOptions;
        assistantBuf = "";
        reasoningBuf = "";
        assistantMsgId = tempId();

        // If the previous send failed, drop that unsent message before adding
        // the new one instead of leaving it behind.
        if (error !== null) discardFailedExchange();

        const convId = conversationIdRef;
        const isNew = convId === null;

        const url = isNew ? getCreateChatUrl() : getSendChatMessageUrl(convId);
        const body = isNew
            ? JSON.stringify({
                  prompt,
                  model: sendOptions.model ?? options.model ?? null,
                  reasoning: sendOptions.reasoning ?? null,
                  title: options.title ?? null,
              } satisfies ChatRequest)
            : JSON.stringify({
                  prompt,
                  reasoning: sendOptions.reasoning ?? null,
              } satisfies MessageRequest);

        const userMsg: ConversationMessage = {
            id: tempId(),
            role: "user",
            content: prompt,
            created_at: new Date().toISOString(),
        };

        if (isNew) {
            conversationId = null;
            messages = [userMsg];
            isLoading = true;
            isStreaming = true;
            isSaved = false;
            error = null;
            errorValue = null;
        } else {
            messages = [...messages, userMsg];
            isLoading = true;
            isStreaming = true;
            isSaved = false;
            error = null;
            errorValue = null;
        }

        const { close } = createSseConnection(
            url,
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body,
            },
            {
                doneSignal: null,
                onOpen: () => {
                    isLoading = false;
                },
                onMessage: (data, event) => {
                    switch (event) {
                        case "conversation_created": {
                            conversationIdRef = data;
                            conversationId = data;
                            break;
                        }
                        case "reasoning": {
                            reasoningBuf += data;
                            const reasoning = reasoningBuf;
                            const id = assistantMsgId;
                            const last = messages[messages.length - 1];
                            if (last?.role === "assistant") {
                                messages = [
                                    ...messages.slice(0, -1),
                                    { ...last, reasoning },
                                ];
                            } else {
                                messages = [
                                    ...messages,
                                    {
                                        id,
                                        role: "assistant",
                                        content: "",
                                        reasoning,
                                        created_at: new Date().toISOString(),
                                    },
                                ];
                            }
                            break;
                        }
                        case "message": {
                            assistantBuf += data;
                            const content = assistantBuf;
                            const id = assistantMsgId;
                            const last = messages[messages.length - 1];
                            if (last?.role === "assistant") {
                                messages = [
                                    ...messages.slice(0, -1),
                                    { ...last, content },
                                ];
                            } else {
                                messages = [
                                    ...messages,
                                    {
                                        id,
                                        role: "assistant",
                                        content,
                                        created_at: new Date().toISOString(),
                                    },
                                ];
                            }
                            break;
                        }
                        case "saved": {
                            isSaved = true;
                            isLoading = false;
                            isStreaming = false;
                            onSaved?.(data);
                            break;
                        }
                        case "error": {
                            isLoading = false;
                            isStreaming = false;
                            const value = toErrorValue(data);
                            errorValue = value;
                            error = isProblemSchema(value)
                                ? value.detail
                                : data;
                            onError?.(error ?? "");
                            closeRef = null;
                            break;
                        }
                    }
                },
                onError: (err) => {
                    isLoading = false;
                    isStreaming = false;
                    errorValue =
                        "problem" in err
                            ? (err as SseProblemError).problem
                            : err;
                    error = err.message || "An error occurred while streaming.";
                    onError?.(error ?? "");
                    closeRef = null;
                },
                onClose: (incomplete) => {
                    isLoading = false;
                    isStreaming = false;
                    if (incomplete && !error) {
                        error = "Stream ended unexpectedly.";
                        errorValue = new Error(error);
                    }
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
        isStreaming = false;
    }

    /**
     * Re-send the most recent user message after a failure.
     *
     * Drops the failed user message (and any partial assistant reply streamed
     * before the error) so the resend doesn't duplicate the exchange, then
     * replays it with the same options.
     */
    function retry() {
        let failed: ConversationMessage | undefined;
        for (let i = messages.length - 1; i >= 0; i--) {
            if (messages[i]?.role === "user") {
                failed = messages[i];
                break;
            }
        }
        if (!failed) return;
        // `send` discards the stale failed exchange before re-adding it.
        send(failed.content, lastSendOptions);
    }

    function reset() {
        closeRef?.();
        closeRef = null;
        assistantBuf = "";
        reasoningBuf = "";
        conversationIdRef = null;
        conversationId = null;
        messages = [];
        isLoading = false;
        isStreaming = false;
        isSaved = false;
        error = null;
        errorValue = null;
    }

    function enterConversation(id: string, msgs: ConversationMessage[] = []) {
        closeRef?.();
        closeRef = null;
        assistantBuf = "";
        reasoningBuf = "";
        conversationIdRef = id;
        conversationId = id;
        messages = msgs;
        isLoading = false;
        isStreaming = false;
        isSaved = true;
        error = null;
        errorValue = null;
    }

    return {
        get conversationId() {
            return conversationId;
        },
        get messages() {
            return messages;
        },
        get isLoading() {
            return isLoading;
        },
        get isStreaming() {
            return isStreaming;
        },
        get isSaved() {
            return isSaved;
        },
        get error() {
            return error;
        },
        get errorValue() {
            return errorValue;
        },
        send,
        stop,
        retry,
        reset,
        enterConversation,
    };
}
