import {
    MODEL_STORAGE_KEYS,
    loadStoredModel,
    storeModel,
} from "$lib/ai/model-storage.js";

export type ChatboxModelSettingsForModel = Record<string, string | null>;
export type ChatboxModelSettings = Record<string, ChatboxModelSettingsForModel>;

class ChatboxStore {
    prompt = $state("");
    /** Id of the message currently being edited, or `null` when not editing. */
    editingMessageId = $state<string | null>(null);
    model = $state<string | null>(loadStoredModel(MODEL_STORAGE_KEYS.chat));
    chatboxAnchor = $state<HTMLElement | null>(null);
    chatboxHeight = $state(0);
    followButtonVisible = $state(false);
    followAgentHandler = $state<(() => void) | null>(null);
    modelSettings = $state<ChatboxModelSettings>({});

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    /** Enter edit mode for a message, loading its content into the prompt. */
    startEditing(id: string, content: string) {
        this.editingMessageId = id;
        this.prompt = content;
    }
    /**
     * Leave edit mode. Intentionally leaves the prompt untouched so cancelling
     * an edit keeps whatever the user has typed.
     */
    stopEditing() {
        this.editingMessageId = null;
    }
    setModel(model: string | null) {
        this.model = model;
        storeModel(MODEL_STORAGE_KEYS.chat, model);
    }
    setChatboxAnchor(el: HTMLElement | null) {
        this.chatboxAnchor = el;
    }
    setChatboxHeight(height: number) {
        this.chatboxHeight = height;
    }
    setFollowButtonVisible(visible: boolean) {
        this.followButtonVisible = visible;
    }
    setFollowAgentHandler(handler: (() => void) | null) {
        this.followAgentHandler = handler;
    }

    setModelSetting(modelId: string, key: string, value: string | null) {
        this.modelSettings = {
            ...this.modelSettings,
            [modelId]: { ...(this.modelSettings[modelId] ?? {}), [key]: value },
        };
    }

    getModelSettings(modelId?: string): ChatboxModelSettingsForModel {
        if (!modelId) return {};
        return this.modelSettings[modelId] ?? {};
    }

    getModelSetting(modelId: string | undefined, key: string): string | null {
        if (!modelId) return null;
        return this.modelSettings[modelId]?.[key] ?? null;
    }
}

export const chatboxStore = new ChatboxStore();
