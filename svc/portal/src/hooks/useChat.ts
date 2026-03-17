"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { createSseConnection } from "@/lib/sse";
import { apiService } from "@/services/api/client";

export const apiKeys = {
  chatStream: () => apiService.getUrl("/chats"),
};

export type ChatMessage = {
  role: "user" | "assistant";
  content: string;
};

type ChatStreamState = {
  conversationId: string | null;
  messages: ChatMessage[];
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

  useEffect(() => {
    return () => {
      if (closeRef.current) {
        closeRef.current();
      }
    };
  }, []);

  const send = useCallback(
    (prompt: string) => {
      if (closeRef.current) {
        closeRef.current();
        closeRef.current = null;
      }

      const url = apiKeys.chatStream();

      assistantBufRef.current = "";

      setState({
        ...initialState,
        messages: [{ role: "user", content: prompt }],
        isLoading: true,
        isStreaming: true,
      });

      const { close } = createSseConnection(
        url.toString(),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            prompt,
            model: options.model ?? undefined,
            title: options.title ?? undefined,
          }),
        },
        {
          doneSignal: null,
          onOpen: () => {
            setState((prev) => ({ ...prev, isLoading: false }));
          },
          onMessage: (data, event) => {
            switch (event) {
              case "conversation_created": {
                const conversationId = data;
                setState((prev) => ({ ...prev, conversationId }));
                options.onConversationCreated?.(conversationId);
                break;
              }

              case "message": {
                assistantBufRef.current += data;
                const content = assistantBufRef.current;

                setState((prev) => {
                  const messages = [...prev.messages];
                  const lastMsg = messages[messages.length - 1];

                  if (lastMsg && lastMsg.role === "assistant") {
                    messages[messages.length - 1] = {
                      role: "assistant",
                      content,
                    };
                  } else {
                    messages.push({ role: "assistant", content });
                  }

                  return { ...prev, messages };
                });
                break;
              }

              case "done": {
                assistantBufRef.current += data;
                const content = assistantBufRef.current;

                setState((prev) => {
                  const messages = [...prev.messages];
                  const lastMsg = messages[messages.length - 1];

                  if (lastMsg && lastMsg.role === "assistant") {
                    messages[messages.length - 1] = {
                      role: "assistant",
                      content,
                    };
                  } else {
                    messages.push({ role: "assistant", content });
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
    [options.model, options.title],
  );

  const stop = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
      setState((prev) => ({
        ...prev,
        isLoading: false,
        isStreaming: false,
      }));
    }
  }, []);

  const reset = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
    }
    assistantBufRef.current = "";
    setState(initialState);
  }, []);

  return {
    ...state,
    send,
    stop,
    reset,
  };
}
