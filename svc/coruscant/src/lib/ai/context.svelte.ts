import { getContext, setContext } from "svelte";
import type { SendChatOptions } from "$lib/ai/chat.svelte.js";
import type { StartImageOptions } from "$lib/ai/image.svelte.js";
import type { Model } from "$lib/ai/models.svelte.js";
import type { StartVideoOptions } from "$lib/ai/video.svelte.js";
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
    retryChat: () => void;
    enterConversation: (id: string, messages?: ConversationMessage[]) => void;
    sendChat: (prompt: string, options?: SendChatOptions) => void;
    readonly generatedImage: string | null;
    readonly isImageLoading: boolean;
    readonly isImageDone: boolean;
    readonly imageError: string | null;
    stopImage: () => void;
    resetImage: () => void;
    startImage: (prompt: string, options?: StartImageOptions) => void;
    readonly generatedVideo: string | null;
    readonly isVideoLoading: boolean;
    readonly isVideoDone: boolean;
    readonly videoError: string | null;
    stopVideo: () => void;
    resetVideo: () => void;
    startVideo: (prompt: string, options?: StartVideoOptions) => void;
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
