import { authenticatedFetch } from "./fetch";

export type SseMessageHandler = (data: string, event?: MessageEvent) => void;
export type SseErrorHandler = (error: Error) => void;
export type SseOpenHandler = () => void;

export interface SseOptions {
  onMessage?: SseMessageHandler;
  onOpen?: SseOpenHandler;
  onError?: SseErrorHandler;
}

export function createSseConnection(
  url: string,
  options: SseOptions = {},
  init?: RequestInit
) {
  const controller = new AbortController();
  const signal = controller.signal;

  (async () => {
    try {
      const response = await authenticatedFetch(url, {
        ...init,
        signal,
      });

      if (!response.ok || !response.body) {
        throw new Error(
          `Streaming request failed with status ${response.status}`
        );
      }

      options.onOpen?.();

      const reader = response.body.getReader();
      const decoder = new TextDecoder("utf-8");
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });

        const parts = buffer.split("\n");
        buffer = parts.pop() ?? "";

        for (const part of parts) {
          const trimmed = part.trim();
          if (!trimmed) continue;

          options.onMessage?.(trimmed);
        }
      }

      const remaining = buffer.trim();
      if (remaining) {
        options.onMessage?.(remaining);
      }
    } catch (err: unknown) {
      if (signal.aborted) {
        return;
      }

      const error =
        err instanceof Error ? err : new Error("Unknown streaming error");
      options.onError?.(error);
    }
  })();

  const close = () => {
    controller.abort();
  };

  return { close };
}
