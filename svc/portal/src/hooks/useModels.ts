import { useCallback, useEffect, useState } from "react";

export type ChatboxVariant = "chat" | "image" | "video";

export interface ModelsFromAPI {
  label: string;
  value: string | null;
  variant: string;
}

interface UseChatboxOptions {
  variant: ChatboxVariant;
}

const fakeDelay = (ms: number) =>
  new Promise<void>((resolve) => setTimeout(resolve, ms));

export function useModels({ variant }: UseChatboxOptions) {
  const [models, setModels] = useState<ModelsFromAPI[] | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const loadModels = useCallback(async () => {
    setIsLoading(true);
    try {
      await fakeDelay(2000);

      const fakeModels = [
        {
          label: "GPT Image 1",
          value: "gpt-image-1",
          variant: "image",
          settings: [],
        },
        { label: "GPT Image 1.5", value: "gpt-image-1.5", variant: "image" },
        {
          label: "GPT Image 1 mini",
          value: "gpt-image-1-mini",
          variant: "image",
        },
        {
          label: "Gemini 2.5 Flash image",
          value: "gemini-2.5-flash-image",
          variant: "image",
        },
        {
          label: "Gemini 3 Pro image",
          value: "gemini-3-pro-image",
          variant: "image",
        },
      ];

      const filtered = fakeModels.filter((m) => m.variant === variant);
      setModels(filtered);
    } finally {
      setIsLoading(false);
    }
  }, [variant]);

  useEffect(() => {
    loadModels();
  }, []);

  return {
    models,
    isLoading,
    loadModels,
  };
}
