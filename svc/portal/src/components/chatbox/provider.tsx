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
  new Promise<void>((resolve) => setTimeout(resolve, ms));

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

      const _ = model || DEFAULT_ENHANCE_MODEL;
      // const res = await enhancePrompt({
      //   body: { original_prompt: promptText, model: selectedModel },
      // });

      setIsEnhancePending(true);
      await fakeDelay(2000);
      setIsEnhancePending(false);

      toast("Prompt has been enhanced", {
        description: "Your prompt has been enhanced.",
        position: "top-center",
        action: {
          label: "Undo",
          onClick: () => setPrompt(prompt),
        },
      });

      return `Enhanced prompt: ${prompt}`;
    },
    [],
  );

  const handleGenerateImage = useCallback(async () => {
    const { prompt, model, modelSettings } = useChatboxStore.getState();

    console.log("SETTINGS:", modelSettings);

    if (!prompt) return;

    const _ = model || DEFAULT_IMAGE_MODEL;

    setIsGeneratePending(true);
    await fakeDelay(2000);
    setIsGeneratePending(false);
  }, []);

  const handleEditImage = useCallback(async () => {
    const { prompt, uploadedImages } = useChatboxStore.getState();
    useChatboxStore.getState();

    if (!prompt || uploadedImages.length === 0) return;

    setIsEditPending(true);
    await fakeDelay(2000);
    setIsEditPending(false);
  }, []);

  useEffect(() => {
    void loadModels(variant);
  }, [variant, loadModels]);

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
