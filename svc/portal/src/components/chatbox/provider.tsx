"use client";

import {
  createContext,
  type Dispatch,
  type ReactNode,
  type SetStateAction,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { toast } from "sonner";
import { type Model, useModels } from "@/hooks/useModels";
import { useChatboxStore } from "@/store/chatbox.store";

export type ChatboxVariant = "chat" | "image" | "video";

export interface ChatboxContextValue {
  variant: ChatboxVariant;
  setVariant: Dispatch<SetStateAction<ChatboxVariant>>;

  showSettings: boolean;
  setShowSettings: Dispatch<SetStateAction<boolean>>;

  allowImageUploads: boolean;
  setAllowImageUploads: Dispatch<SetStateAction<boolean>>;

  models: Model[];
  isModelsLoading: boolean;
  loadModels: (variant?: string | undefined) => Promise<void>;

  enhancePrompt: (prompt: string) => Promise<string>;
  isEnhancePending: boolean;

  generateImage: () => Promise<void>;
  isGeneratePending: boolean;

  editImage: () => Promise<void>;
  isEditPending: boolean;

  hasPendingWork: boolean;
}

const ChatboxContext = createContext<ChatboxContextValue | undefined>(
  undefined,
);

export interface ChatboxProviderProps {
  defaultVariant?: ChatboxVariant;
  children: ReactNode;
}

export const DEFAULT_IMAGE_MODEL = "gemini-2.5-flash-image" as const;
export const DEFAULT_ENHANCE_MODEL = "gemini-2.5-flash" as const;

const fakeDelay = (ms: number) =>
  new Promise<void>((resolve, reject) => {
    setTimeout(() => {
      // ~33% chance to fail
      if (Math.random() < 1 / 3) {
        reject(new Error("fakeDelay failed intentionally (33% chance)"));
        return;
      }
      resolve();
    }, ms);
  });

export function ChatboxProvider({
  defaultVariant = "image",
  children,
}: ChatboxProviderProps) {
  const [variant, setVariant] = useState<ChatboxVariant>(defaultVariant);
  const [showSettings, setShowSettings] = useState<boolean>(true);
  const [allowImageUploads, setAllowImageUploads] = useState<boolean>(true);

  const [isEnhancePending, setIsEnhancePending] = useState<boolean>(false);
  const [isGeneratePending, setIsGeneratePending] = useState<boolean>(false);
  const [isEditPending, setIsEditPending] = useState<boolean>(false);

  const { models, isLoading: isModelsLoading, loadModels } = useModels();

  const handleEnhancePrompt = useCallback(
    async (prompt: string, model?: string) => {
      const { setPrompt } = useChatboxStore.getState();

      const selectedModel = model || DEFAULT_ENHANCE_MODEL;

      setIsEnhancePending(true);
      try {
        const request = fakeDelay(2000);
        toast.promise(request, {
          position: "top-center",
          loading: "Enhancing prompt...",
          duration: 15000,
          success: {
            message: "Prompt has been enhanced",
            description: "Your prompt has been enhanced.",
            action: {
              label: "Undo",
              onClick: () => setPrompt(prompt),
            },
          },
          error: (e) => ({
            message: "Failed to enhance prompt",
            description: `${e instanceof Error ? e.message : String(e)}`,
          }),
        });
        await request;
      } finally {
        setIsEnhancePending(false);
      }

      return `Enhanced prompt: ${prompt}`;
    },
    [],
  );

  const handleGenerateImage = useCallback(async () => {
    const { prompt, model, modelSettings } = useChatboxStore.getState();

    if (!prompt) return;

    const selectedModel = model || DEFAULT_IMAGE_MODEL;
    const selectedModelSettings = modelSettings[selectedModel];

    setIsGeneratePending(true);
    try {
      const res = fakeDelay(2000);
      toast.promise(res, {
        position: "top-center",
        loading: "Generating image(s)...",
        success: () => ({
          message: `Generated image(s)!`,
          description: "Saved the image(s) as 123.png",
        }),
        error: (e) => ({
          message: "Failed to generate image(s)!",
          description: `${e instanceof Error ? e.message : String(e)}`,
        }),
      });
      await res;
    } finally {
      setIsGeneratePending(false);
    }
  }, []);

  const handleEditImage = useCallback(async () => {
    const { prompt, model, uploadedImages, modelSettings } =
      useChatboxStore.getState();

    if (!prompt || uploadedImages.length === 0) return;

    const selectedModel = model || DEFAULT_IMAGE_MODEL;
    const selectedModelSettings = modelSettings[selectedModel];

    setIsEditPending(true);
    try {
      const res = fakeDelay(2000);
      toast.promise(res, {
        position: "top-center",
        loading: "Generating image(s)...",
        success: () => ({
          message: "Failed to generate image(s)!",
          description: "Saved the image(s) as 123.png",
        }),
        error: (e) => ({
          message: "Failed to generate image(s)!",
          description: `${e instanceof Error ? e.message : String(e)}`,
        }),
      });
      await res;
    } finally {
      setIsEditPending(false);
    }
  }, []);

  useEffect(() => {
    void loadModels(variant);
  }, [variant]);

  const value = useMemo<ChatboxContextValue>(
    () => ({
      variant,
      setVariant,

      showSettings,
      setShowSettings,

      allowImageUploads,
      setAllowImageUploads,

      models,
      isModelsLoading,
      loadModels,

      enhancePrompt: handleEnhancePrompt,
      isEnhancePending,

      generateImage: handleGenerateImage,
      isGeneratePending,

      editImage: handleEditImage,
      isEditPending,

      hasPendingWork: isEnhancePending || isGeneratePending || isEditPending,
    }),
    [
      variant,
      showSettings,
      allowImageUploads,
      models,
      isModelsLoading,
      loadModels,
      isEnhancePending,
      isGeneratePending,
      isEditPending,
    ],
  );

  return (
    <ChatboxContext.Provider value={value}>{children}</ChatboxContext.Provider>
  );
}

export function useChatbox(): ChatboxContextValue {
  const ctx = useContext(ChatboxContext);

  if (!ctx) {
    throw new Error("useChatbox must be used within a <ChatboxProvider>");
  }

  return ctx;
}
