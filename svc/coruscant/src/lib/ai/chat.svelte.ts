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

export function createChat(options: UseChatOptions = {}) {
    let conversationId = $state<string | null>(null);
    let messages = $state<ConversationMessage[]>([]);
    let isLoading = $state(false);
    let isStreaming = $state(false);
    let isSaved = $state(false);
    let error = $state<string | null>(null);

    let closeRef: (() => void) | null = null;
    let assistantBuf = "";
    let assistantMsgId = tempId();
    let conversationIdRef: string | null = null;

    // Callbacks via refs to avoid stale closures
    let onSaved = options.onSaved;
    let onError = options.onError;

    $effect(() => {
        return () => closeRef?.();
    });

    function send(prompt: string) {
        closeRef?.();
        closeRef = null;
        assistantBuf = "";
        assistantMsgId = tempId();

        let streamDone = false;
        const convId = conversationIdRef;
        const isNew = convId === null;

        const url = isNew ? getCreateChatUrl() : getSendChatMessageUrl(convId);
        const body = isNew
            ? JSON.stringify({
                  prompt,
                  model: options.model ?? null,
                  title: options.title ?? null,
              } satisfies ChatRequest)
            : JSON.stringify({ prompt } satisfies MessageRequest);

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
                        case "done": {
                            // `done` carries the full, authoritative response,
                            // so replace the streamed buffer rather than
                            // appending to avoid duplicating the message.
                            assistantBuf = data;
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
                            isStreaming = false;
                            streamDone = true;
                            break;
                        }
                        case "saved": {
                            isSaved = true;
                            isLoading = false;
                            onSaved?.(data);
                            break;
                        }
                        case "error": {
                            isLoading = false;
                            isStreaming = false;
                            streamDone = true;
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
                    // Always finalize streaming state when the connection
                    // closes. With `doneSignal: null` a clean close reports
                    // `incomplete === false`, so without this the spinner would
                    // hang whenever the server omits an explicit `done` event.
                    isLoading = false;
                    isStreaming = false;
                    if (!streamDone && (incomplete || !error)) {
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
