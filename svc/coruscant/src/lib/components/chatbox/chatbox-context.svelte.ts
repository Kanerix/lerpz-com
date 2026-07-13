import { getContext, setContext } from "svelte";
import type { Model } from "$lib/ai/models.svelte.js";
import type { ChatboxModelSettingsForModel } from "$lib/components/chatbox/chatbox.store.svelte.js";

export type ChatboxSubmitArgs = {
    prompt: string;
    model: string | null;
    modelSettings: ChatboxModelSettingsForModel;
};

export type ChatboxContextValue = {
    readonly showSettings: boolean;
    setShowSettings: (v: boolean) => void;
    readonly models: Model[];
    readonly isModelsLoading: boolean;
    loadModels: (mode?: string) => Promise<void>;
    submit: () => Promise<void>;
    readonly isSubmitPending: boolean;
    enhance?: (prompt: string) => Promise<string>;
    readonly isEnhancePending: boolean;
    readonly isPending: boolean;
};

const CHATBOX_KEY = Symbol("chatbox");

export function setChatboxContext(ctx: ChatboxContextValue) {
    setContext(CHATBOX_KEY, ctx);
}

export function getChatboxContext(): ChatboxContextValue {
    const ctx = getContext<ChatboxContextValue>(CHATBOX_KEY);
    if (!ctx)
        throw new Error("getChatboxContext must be used inside <Chatbox>");
    return ctx;
}
