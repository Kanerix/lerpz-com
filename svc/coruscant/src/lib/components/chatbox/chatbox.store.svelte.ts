export type ChatboxUploadedImage = {
    id: string;
    file: File;
    previewUrl: string;
    addedAt: number;
};

export type ChatboxModelSettingsForModel = Record<string, string | null>;
export type ChatboxModelSettings = Record<string, ChatboxModelSettingsForModel>;

export const REASONING_KEY = "reasoning";
export const REASONING_LEVELS = [
    { value: "none", label: "None" },
    { value: "low", label: "Low" },
    { value: "medium", label: "Medium" },
    { value: "high", label: "High" },
] as const;

export const DEFAULT_REASONING_LEVEL = "medium";

function generateImageId(): string {
    return `img_${Math.random().toString(36).slice(2)}_${Date.now()}`;
}

class ChatboxStore {
    prompt = $state("");
    model = $state<string | null>(null);
    chatboxAnchor = $state<HTMLElement | null>(null);
    autoAnalyze = $state(true);
    modelSettings = $state<ChatboxModelSettings>({});
    uploadedImages = $state<ChatboxUploadedImage[]>([]);

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    setModel(model: string | null) {
        this.model = model;
    }
    setChatboxAnchor(el: HTMLElement | null) {
        this.chatboxAnchor = el;
    }
    setAutoAnalyze(v: boolean) {
        this.autoAnalyze = v;
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

    addUploadedImages(files: File[]) {
        const mapped: ChatboxUploadedImage[] = files.map((file) => ({
            id: generateImageId(),
            file,
            previewUrl: URL.createObjectURL(file),
            addedAt: Date.now(),
        }));
        this.uploadedImages = [...this.uploadedImages, ...mapped];
    }

    removeUploadedImage(id: string) {
        this.uploadedImages = this.uploadedImages.filter((img) => {
            if (img.id === id) {
                URL.revokeObjectURL(img.previewUrl);
                return false;
            }
            return true;
        });
    }

    clearUploadedImages() {
        this.uploadedImages.forEach((img) => {
            URL.revokeObjectURL(img.previewUrl);
        });
        this.uploadedImages = [];
    }
}

export const chatboxStore = new ChatboxStore();
