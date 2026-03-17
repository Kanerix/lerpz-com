"use client";

import {
  createContext,
  type Dispatch,
  type ReactNode,
  type SetStateAction,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";
import type { Model } from "@/hooks/useModels";
import {
  type ChatboxModelSettingsForModel,
  type ChatboxUploadedImage,
  useChatboxStore,
} from "./store";

export type ChatboxMode = "chat" | "image" | "video";

export interface ChatboxSubmitArgs {
  prompt: string;
  mode: ChatboxMode;
  model: string | null;
  modelSettings: ChatboxModelSettingsForModel;
  images: ChatboxUploadedImage[];
}

export interface ChatboxContextValue {
  mode: ChatboxMode;
  setMode: Dispatch<SetStateAction<ChatboxMode>>;

  showSettings: boolean;
  setShowSettings: Dispatch<SetStateAction<boolean>>;

  allowImageUploads: boolean;
  setAllowImageUploads: Dispatch<SetStateAction<boolean>>;

  models: Model[];
  isModelsLoading: boolean;
  loadModels: (mode?: string | undefined) => Promise<void>;

  submit: () => void | Promise<void>;
  isSubmitting: boolean;

  enhancePrompt?: (prompt: string) => Promise<string>;
  isEnhancePending: boolean;
}

const ChatboxContext = createContext<ChatboxContextValue | undefined>(
  undefined,
);

export interface ChatboxProviderProps {
  onSubmit: (args: ChatboxSubmitArgs) => void | Promise<void>;
  onEnhancePrompt?: (prompt: string) => Promise<string>;
  models?: Model[];
  isModelsLoading?: boolean;
  loadModels?: (mode?: string) => Promise<void>;
  defaultMode?: ChatboxMode;
  children: ReactNode;
}

export const DEFAULT_IMAGE_MODEL = "gemini-2.5-flash-image" as const;

const noopLoadModels = async () => {};

export function ChatboxProvider({
  onSubmit,
  onEnhancePrompt,
  models = [],
  isModelsLoading = false,
  loadModels = noopLoadModels,
  defaultMode = "chat",
  children,
}: ChatboxProviderProps) {
  const [mode, setMode] = useState<ChatboxMode>(defaultMode);
  const [showSettings, setShowSettings] = useState<boolean>(true);
  const [allowImageUploads, setAllowImageUploads] = useState<boolean>(true);

  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);
  const [isEnhancePending, setIsEnhancePending] = useState<boolean>(false);

  const handleSubmit = useCallback(async () => {
    const { prompt, model, uploadedImages, getModelSettings, setPrompt, clearUploadedImages } =
      useChatboxStore.getState();

    const trimmed = prompt.trim();
    if (!trimmed) return;

    const args: ChatboxSubmitArgs = {
      prompt: trimmed,
      mode,
      model,
      modelSettings: getModelSettings(model ?? undefined),
      images: uploadedImages,
    };

    setIsSubmitting(true);
    try {
      await onSubmit(args);
      setPrompt("");
      clearUploadedImages();
    } finally {
      setIsSubmitting(false);
    }
  }, [mode, onSubmit]);

  const handleEnhancePrompt = useCallback(
    async (prompt: string): Promise<string> => {
      if (!onEnhancePrompt) return prompt;

      setIsEnhancePending(true);
      try {
        return await onEnhancePrompt(prompt);
      } finally {
        setIsEnhancePending(false);
      }
    },
    [onEnhancePrompt],
  );

  const value = useMemo<ChatboxContextValue>(
    () => ({
      mode,
      setMode,

      showSettings,
      setShowSettings,

      allowImageUploads,
      setAllowImageUploads,

      models,
      isModelsLoading,
      loadModels,

      submit: handleSubmit,
      isSubmitting,

      enhancePrompt: onEnhancePrompt ? handleEnhancePrompt : undefined,
      isEnhancePending,
    }),
    [
      mode,
      showSettings,
      allowImageUploads,
      models,
      isModelsLoading,
      loadModels,
      handleSubmit,
      isSubmitting,
      onEnhancePrompt,
      handleEnhancePrompt,
      isEnhancePending,
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
