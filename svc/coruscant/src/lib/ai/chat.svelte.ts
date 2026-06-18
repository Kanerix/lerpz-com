import { createSseConnection } from "$lib/http/sse.js";
import {
    getCreateChatUrl,
    getSendChatMessageUrl,
} from "$lib/api/chats/chats.js";
import type {
    ChatRequest,
    ConversationMessage,
    MessageRequest,
} from "$lib/api/models/index.js";

export type ChatMessage = ConversationMessage;

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
    /** Model override for new conversations (ignored for existing ones). */
    model?: string | null;
    /**
     * Reasoning level for reasoning-capable models (e.g. `low`, `medium`,
     * `high`, or `none` to disable). Omit (or pass `null`) to use the model's
     * default behaviour.
     */
    reasoning?: string | null;
};

export function createChat(options: UseChatOptions = {}) {
    let conversationId = $state<string | null>(null);
    let messages = $state<ConversationMessage[]>([]);
    let isLoading = $state(false);
    let isStreaming = $state(false);
    let isSaved = $state(false);
    let error = $state<string | null>(null);

    let closeRef: (() => void) | null = null;
    let assistantBuf = "";
    let reasoningBuf = "";
    let assistantMsgId = tempId();
    let conversationIdRef: string | null = null;

    // Callbacks via refs to avoid stale closures
    let onSaved = options.onSaved;
    let onError = options.onError;

    $effect(() => {
        return () => closeRef?.();
    });

    function send(prompt: string, sendOptions: SendChatOptions = {}) {
        closeRef?.();
        closeRef = null;
        assistantBuf = "";
        reasoningBuf = "";
        assistantMsgId = tempId();

        const convId = conversationIdRef;
        const isNew = convId === null;

        const url = isNew ? getCreateChatUrl() : getSendChatMessageUrl(convId);
        const reasoning = sendOptions.reasoning ?? null;
        const body = isNew
            ? JSON.stringify({
                  prompt,
                  model: sendOptions.model ?? options.model ?? null,
                  reasoning,
                  title: options.title ?? null,
              } satisfies ChatRequest)
            : JSON.stringify({ prompt, reasoning } satisfies MessageRequest);

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
        } else {
            messages = [...messages, userMsg];
            isLoading = true;
            isStreaming = true;
            isSaved = false;
            error = null;
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
                            error = data;
                            onError?.(data);
                            closeRef = null;
                            break;
                        }
                    }
                },
                onError: (err) => {
                    isLoading = false;
                    isStreaming = false;
                    error = err.message || "An error occurred while streaming.";
                    onError?.(error ?? "");
                    closeRef = null;
                },
                onClose: (incomplete) => {
                    isLoading = false;
                    isStreaming = false;
                    if (incomplete && !error) {
                        error = "Stream ended unexpectedly.";
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
        send,
        stop,
        reset,
        enterConversation,
    };
}
