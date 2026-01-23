"use client";

import {
  createContext,
  type Dispatch,
  type ReactNode,
  type SetStateAction,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { useEnhance } from "@/hooks/useEnhance";
import { useModels } from "@/hooks/useModels";

export type ChatboxVariant = "chat" | "image" | "video";

export interface ChatboxModels {
  label: string;
  value: string | null;
  variant: string;
}

export interface ChatboxContextValue {
  variant: ChatboxVariant;
  setVariant: Dispatch<SetStateAction<ChatboxVariant>>;

  models: ChatboxModels[];
  isModelsLoading: boolean;
  loadModels: (variant?: string | undefined) => Promise<void>;

  enhance: (prompt: string) => Promise<string>;
  isEnhanceLoading: boolean;

  showSettings: boolean;
  setShowSettings: Dispatch<SetStateAction<boolean>>;

  // NEW: setting to allow image uploads
  allowImageUploads: boolean;
  setAllowImageUploads: Dispatch<SetStateAction<boolean>>;
}

const ChatboxContext = createContext<ChatboxContextValue | undefined>(
  undefined,
);

export interface ChatboxProviderProps {
  defaultVariant?: ChatboxVariant;
  children: ReactNode;
}

export function ChatboxProvider({
  defaultVariant = "chat",
  children,
}: ChatboxProviderProps) {
  const [variant, setVariant] = useState<ChatboxVariant>(defaultVariant);
  const [showSettings, setShowSettings] = useState<boolean>(true);
  const [allowImageUploads, setAllowImageUploads] = useState<boolean>(true);

  const { models, isLoading: isModelsLoading, loadModels } = useModels();
  const { enhance, isLoading: isEnhanceLoading } = useEnhance();

  useEffect(() => {
    loadModels(variant);
  }, [variant, loadModels]);

  const value = useMemo<ChatboxContextValue>(
    () => ({
      variant,
      setVariant,

      models,
      isModelsLoading,
      loadModels,

      enhance,
      isEnhanceLoading,

      showSettings,
      setShowSettings,

      allowImageUploads,
      setAllowImageUploads,
    }),
    [
      variant,
      models,
      isModelsLoading,
      loadModels,
      enhance,
      isEnhanceLoading,
      showSettings,
      allowImageUploads,
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
