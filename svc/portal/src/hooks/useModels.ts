import { useCallback, useEffect, useState } from "react";

export type ChatboxVariant = "chat" | "image" | "video";

export interface ModelsFromAPI {
  label: string;
  value: string | null;
  variant: string;
}

const fakeDelay = (ms: number) =>
  new Promise<void>((resolve) => setTimeout(resolve, ms));

export function useModels() {
  const [models, setModels] = useState<ModelsFromAPI[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const loadModels = useCallback(async (variant?: string) => {
    setIsLoading(true);
    try {
      await fakeDelay(2000);

      const fakeModels = [
        {
          label: "GPT 5.2",
          value: "gpt-5.2",
          variant: "chat",
        },
        {
          label: "Opus 4.5",
          value: "claude-opus-4.5",
          variant: "chat",
        },
        {
          label: "GPT Image 1",
          value: "gpt-image-1",
          variant: "image",
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
        {
          label: "Veo 3",
          value: "google-veo-pro",
          variant: "video",
        },
        {
          label: "Sora 2",
          value: "sora-2",
          variant: "video",
        },
      ];

      if (variant) {
        const filtered = fakeModels.filter((m) => m.variant === variant);
        setModels(filtered);
      } else {
        setModels(fakeModels);
      }
    } finally {
      setIsLoading(false);
    }
  }, []);

  return {
    models,
    isLoading,
    loadModels,
  };
}
