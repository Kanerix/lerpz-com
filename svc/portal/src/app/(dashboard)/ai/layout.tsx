"use client";

import { useRouter } from "next/navigation";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useMemo,
} from "react";
import { toast } from "sonner";
import {
  Chatbox,
  ChatboxProvider,
  type ChatboxSubmitArgs,
} from "@/components/chatbox";
import { type ChatMessage, useChat } from "@/hooks/useChat";
import { useImageSse } from "@/hooks/useImage";
import { useModels } from "@/hooks/useModels";

interface AiContextValue {
  // Chat
  chatMessages: ChatMessage[];
  isChatLoading: boolean;
  isChatStreaming: boolean;
  chatError: string | null;
  conversationId: string | null;
  isChatSaved: boolean;
  stopChat: () => void;
  resetChat: () => void;
  enterConversation: (id: string, messages?: ChatMessage[]) => void;

  // Image
  generatedImage: string | null;
  isImageLoading: boolean;
  isImageDone: boolean;
  imageError: string | null;
  stopImage: () => void;
  resetImage: () => void;
}

const AiContext = createContext<AiContextValue | undefined>(undefined);

export function useAi(): AiContextValue {
  const ctx = useContext(AiContext);
  if (!ctx) {
    throw new Error("useAi must be used within the AI layout");
  }
  return ctx;
}

interface LayoutProps {
  children: ReactNode;
}

export default function Layout({ children }: LayoutProps) {
  const router = useRouter();
  const { models, isLoading: isModelsLoading, loadModels } = useModels();

  const chat = useChat({
    onSaved: (convId) => {
      toast.success("Chat saved", {
        description: "Your conversation has been saved.",
      });
      router.replace(`/ai/chats/${convId}`);
    },
    onError: (error) => {
      toast.error("Chat error", {
        position: "top-center",
        description: error,
      });
    },
  });

  const image = useImageSse();

  const handleSubmit = useCallback(
    async (args: ChatboxSubmitArgs) => {
      switch (args.mode) {
        case "chat": {
          chat.send(args.prompt);
          return;
        }
        case "image": {
          image.start(args.prompt);
          return;
        }
        case "video": {
          toast.info("Video generation is not yet available.", {
            position: "top-center",
          });
          return;
        }
      }
    },
    [chat, image],
  );

  const handleEnhancePrompt = useCallback(async (prompt: string) => {
    // TODO: implement real prompt enhancement
    toast.info("Prompt enhancement coming soon.", {
      position: "top-center",
    });
    return prompt;
  }, []);

  const aiValue = useMemo<AiContextValue>(
    () => ({
      chatMessages: chat.messages,
      isChatLoading: chat.isLoading,
      isChatStreaming: chat.isStreaming,
      chatError: chat.error,
      conversationId: chat.conversationId,
      isChatSaved: chat.isSaved,
      stopChat: chat.stop,
      resetChat: chat.reset,
      enterConversation: chat.enterConversation,

      generatedImage: image.image,
      isImageLoading: image.isLoading,
      isImageDone: image.isDone,
      imageError: image.error,
      stopImage: image.stop,
      resetImage: image.reset,
    }),
    [
      chat.messages,
      chat.isLoading,
      chat.isStreaming,
      chat.error,
      chat.conversationId,
      chat.isSaved,
      chat.stop,
      chat.reset,
      chat.enterConversation,
      image.image,
      image.isLoading,
      image.isDone,
      image.error,
      image.stop,
      image.reset,
    ],
  );

  return (
    <ChatboxProvider
      onSubmit={handleSubmit}
      onEnhance={handleEnhancePrompt}
      isStreaming={chat.isStreaming || chat.isLoading || image.isLoading}
      models={models}
      isModelsLoading={isModelsLoading}
      loadModels={loadModels}
    >
      <AiContext.Provider value={aiValue}>
        {children}
        <Chatbox />
      </AiContext.Provider>
    </ChatboxProvider>
  );
}