export type ChatboxModelSettingsForModel = Record<string, string | null>;
export type ChatboxModelSettings = Record<string, ChatboxModelSettingsForModel>;

class ChatboxStore {
    prompt = $state("");
    model = $state<string | null>(null);
    chatboxAnchor = $state<HTMLElement | null>(null);
    chatboxHeight = $state(0);
    followButtonVisible = $state(false);
    followAgentHandler = $state<(() => void) | null>(null);
    modelSettings = $state<ChatboxModelSettings>({});

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    setModel(model: string | null) {
        this.model = model;
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
