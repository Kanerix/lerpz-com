"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { createSseConnection } from "@/lib/sse";
import { apiService } from "@/services/api/client";

export const apiKeys = {
  chatStream: () => apiService.getUrl("/chats"),
};

type ChatStreamState = {
  image: string | null;
  isLoading: boolean;
  isDone: boolean;
  error: string | null;
};

export function useChatSse() {
  const [state, setState] = useState<ChatStreamState>({
    content: null,
    isLoading: false,
    isDone: false,
    error: null,
  });

  const closeRef = useRef<null | (() => void)>(null);

  useEffect(() => {
    return () => {
      if (closeRef.current) {
        closeRef.current();
      }
    };
  }, []);

  const start = useCallback((prompt: string) => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
    }

    const url = apiKeys.imageStream();

    setState({
      image: null,
      isLoading: true,
      isDone: false,
      error: null,
    });

    const { close } = createSseConnection(
      url.toString(),
      {
        onOpen: () => {
          console.log("Image stream connection opened");
        },
        onMessage: (data) => {
          if (data.startsWith("data:")) {
            data = data.slice("data:".length).trimStart();
          }

          setState((prev) => ({
            ...prev,
            image: `data:image/png;base64,${data}`,
          }));
        },
        onError: (error) => {
          console.error("Image stream error:", error);
          setState((prev) => ({
            ...prev,
            isLoading: false,
            isDone: true,
            error: "An error occurred while streaming the image.",
          }));
          closeRef.current = null;
        },
      },
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ prompt }),
      },
    );

    closeRef.current = close;
  }, []);

  const stop = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
      setState((prev) => ({
        ...prev,
        isLoading: false,
        isDone: true,
      }));
    }
  }, []);

  const reset = useCallback(() => {
    if (closeRef.current) {
      closeRef.current();
      closeRef.current = null;
    }
    setState({
      image: null,
      isLoading: false,
      isDone: false,
      error: null,
    });
  }, []);

  return {
    ...state,
    start,
    stop,
    reset,
  };
}
