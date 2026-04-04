"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import type {
  ChatRequest,
  ConversationMessage,
  MessageRequest,
} from "@/services/api/models";
import {
  getCreateChatUrl,
  getSendChatMessageUrl,
} from "@/services/api/chats/chats";
import { createSseConnection } from "@/lib/sse";

// Re-export under the legacy name so all existing consumers keep working.
export type { ConversationMessage as ChatMessage };

// Monotonically-increasing counter used to give streaming messages a stable,
// locally-unique ID before the server assigns a real UUID.
let _nextTempId = 0;
function tempId(): string {
  return `__temp_${++_nextTempId}`;
}

type ChatStreamState = {
  conversationId: string | null;
  messages: ConversationMessage[];
  isLoading: boolean;
  isStreaming: boolean;
  isSaved: boolean;
  error: string | null;
};

const initialState: ChatStreamState = {
  conversationId: null,
  messages: [],
  isLoading: false,
  isStreaming: false,
  isSaved: false,
  error: null,
};

export type UseChatOptions = {
  model?: string;
  title?: string;
  onConversationCreated?: (convId: string) => void;
  onDone?: (content: string) => void;
  onSaved?: (convId: string) => void;
  onError?: (error: string) => void;
};

export function useChat(options: UseChatOptions = {}) {
  const [state, setState] = useState<ChatStreamState>(initialState);
  const closeRef = useRef<null | (() => void)>(null);
  const assistantBufRef = useRef<string>("");
  // Shadow ref so the memoised `send` callback always reads the latest
  // conversationId without declaring it as a dependency.
  const conversationIdRef = useRef<string | null>(null);
  // Stable ID for the assistant message that is currently being streamed.
  const assistantMsgIdRef = useRef<string>(tempId());

  useEffect(() => {
    return () => {
      closeRef.current?.();
    };
  }, []);

  const send = useCallback(
    (prompt: string) => {
      if (closeRef.current) {
        closeRef.current();
        closeRef.current = null;
      }

      assistantBufRef.current = "";
      // Allocate a fresh ID for the assistant reply we are about to receive.
      assistantMsgIdRef.current = tempId();

      const convId = conversationIdRef.current;
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
        // Fresh conversation — reset all state.
        setState({
          ...initialState,
          messages: [userMsg],
          isLoading: true,
          isStreaming: true,
        });
      } else {
        // Continuing — append the new user message to the existing history.
        setState((prev) => ({
          ...prev,
          messages: [...prev.messages, userMsg],
          isLoading: true,
          isStreaming: true,
          isSaved: false,
          error: null,
        }));
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
            setState((prev) => ({ ...prev, isLoading: false }));
          },
          onMessage: (data, event) => {
            switch (event) {
              case "conversation_created": {
                // Only emitted by the create endpoint.
                const conversationId = data;
                conversationIdRef.current = conversationId;
                setState((prev) => ({ ...prev, conversationId }));
                options.onConversationCreated?.(conversationId);
                break;
              }

              case "message": {
                assistantBufRef.current += data;
                const content = assistantBufRef.current;
                const assistantId = assistantMsgIdRef.current;

                setState((prev) => {
                  const messages = [...prev.messages];
                  const last = messages[messages.length - 1];

                  if (last?.role === "assistant") {
                    messages[messages.length - 1] = { ...last, content };
                  } else {
                    messages.push({

                      id: assistantId,
                      role: "assistant",
                      content,
                      created_at: new Date().toISOString(),
                    });
                  }

                  return { ...prev, messages };
                });
                break;
              }

              case "done": {
                assistantBufRef.current += data;
                const content = assistantBufRef.current;
                const assistantId = assistantMsgIdRef.current;

                setState((prev) => {
                  const messages = [...prev.messages];
                  const last = messages[messages.length - 1];

                  if (last?.role === "assistant") {
                    messages[messages.length - 1] = { ...last, content };
                  } else {
                    messages.push({
                      id: assistantId,
                      role: "assistant",
                      content,
                      created_at: new Date().toISOString(),
                    });
                  }

                  return { ...prev, messages, isStreaming: false };
                });

                options.onDone?.(content);
                break;
              }

              case "saved": {
                const conversationId = data;
                setState((prev) => ({
                  ...prev,
                  isSaved: true,
                  isLoading: false,
                }));
                options.onSaved?.(conversationId);
                break;
              }

              case "error": {
                setState((prev) => ({
                  ...prev,
                  isLoading: false,
                  isStreaming: false,
                  error: data,
                }));
                options.onError?.(data);
                closeRef.current = null;
                break;
              }
            }
          },
          onError: (error) => {
            console.error("Chat stream error:", error);
            setState((prev) => ({
              ...prev,
              isLoading: false,
              isStreaming: false,
              error: error.message || "An error occurred while streaming.",
            }));
            options.onError?.(error.message);
            closeRef.current = null;
          },
          onClose: (incomplete) => {
            if (incomplete) {
              setState((prev) => ({
                ...prev,
                isLoading: false,
                isStreaming: false,
                error: prev.error ?? "Stream ended unexpectedly.",
              }));
            }
            closeRef.current = null;
          },
        },
      );

      closeRef.current = close;
    },
    // Intentionally omits callback options — they are captured by closure and
    // are stable in practice (toast / router.replace calls).
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [options.model, options.title],
  );

  const stop = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
      setState((prev) => ({ ...prev, isLoading: false, isStreaming: false }));
    }
  }, []);

  const reset = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
    }
    assistantBufRef.current = "";
    conversationIdRef.current = null;
    setState(initialState);
  }, []);

  /**
   * Load an existing conversation into the hook so that the next `send` call
   * continues it rather than creating a new one. Typically called when the
   * user navigates to `/ai/chats/[id]` from the sidebar.
   */
  const enterConversation = useCallback(
    (id: string, messages: ConversationMessage[] = []) => {
      if (closeRef.current) {
        closeRef.current();
        closeRef.current = null;
      }
      assistantBufRef.current = "";
      conversationIdRef.current = id;
      setState({
        conversationId: id,
        messages,
        isLoading: false,
        isStreaming: false,
        isSaved: true,
        error: null,
      });
    },
    [],
  );

  return {
    ...state,
    send,
    stop,
    reset,
    enterConversation,
  };
}
