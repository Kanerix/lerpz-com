import { getContext, setContext } from "svelte";
import type { Model } from "$lib/ai/models.svelte.js";
import type { ConversationMessage } from "$lib/api/models/index.js";

export const AI_CONTEXT_KEY = Symbol("ai-context");

export type AiContextValue = {
    readonly chatMessages: ConversationMessage[];
    readonly isChatLoading: boolean;
    readonly isChatStreaming: boolean;
    readonly chatError: string | null;
    readonly conversationId: string | null;
    readonly isChatSaved: boolean;
    stopChat: () => void;
    resetChat: () => void;
    enterConversation: (id: string, messages?: ConversationMessage[]) => void;
    sendChat: (prompt: string) => void;
    readonly generatedImage: string | null;
    readonly isImageLoading: boolean;
    readonly isImageDone: boolean;
    readonly imageError: string | null;
    stopImage: () => void;
    resetImage: () => void;
    startImage: (prompt: string) => void;
    readonly models: Model[];
    readonly isModelsLoading: boolean;
    loadModels: (modality?: string) => Promise<void>;
};

export function setAiContext(ctx: AiContextValue) {
    setContext(AI_CONTEXT_KEY, ctx);
}

export function getAiContext(): AiContextValue {
    const ctx = getContext<AiContextValue>(AI_CONTEXT_KEY);
    if (!ctx) throw new Error("getAiContext must be used inside the AI layout");
    return ctx;
}
