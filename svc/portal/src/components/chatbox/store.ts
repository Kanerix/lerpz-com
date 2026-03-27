import { create } from "zustand";

export interface ChatboxUploadedImage {
  id: string;
  file: File;
  previewUrl: string;
  addedAt: number;
}

export type ChatboxModelSettingsForModel = Record<string, string | null>;

export type ChatboxModelSettings = Record<string, ChatboxModelSettingsForModel>;

export interface ChatboxState {
  prompt: string;
  setPrompt: (prompt: string) => void;

  model: string | null;
  setModel: (model: string | null) => void;

  chatboxAnchor: HTMLElement | null;
  setChatboxAnchor: (el: HTMLElement | null) => void;

  autoAnalyze: boolean;
  setAutoAnalyze: (autoAnalyze: boolean) => void;

  modelSettings: ChatboxModelSettings;
  setModelSetting: (
    modelId: string,
    modelKey: string,
    value: string | null,
  ) => void;
  getModelSettings: (modelId?: string) => ChatboxModelSettingsForModel;
  getModelSetting: (
    modelId: string | undefined,
    modelKey: string,
  ) => string | null;

  uploadedImages: ChatboxUploadedImage[];
  addUploadedImages: (images: File[]) => void;
  removeUploadedImage: (id: string) => void;
  clearUploadedImages: () => void;
}

function generateImageId(prefix: string = "img"): string {
  return `${prefix}_${Math.random().toString(36).slice(2)}_${Date.now()}`;
}

export const useChatboxStore = create<ChatboxState>((set, get) => ({
  prompt: "",
  setPrompt: (prompt) => {
    return set({ prompt });
  },

  model: null,
  setModel: (model) => {
    return set({ model });
  },

  chatboxAnchor: null,
  setChatboxAnchor: (el) => {
    return set({ chatboxAnchor: el });
  },

  autoAnalyze: true,
  setAutoAnalyze: (autoAnalyze) => {
    return set({ autoAnalyze });
  },

  modelSettings: {},

  setModelSetting: (modelId, modelKey, value) =>
    set((state) => ({
      modelSettings: {
        ...state.modelSettings,
        [modelId]: {
          ...(state.modelSettings[modelId] ?? {}),
          [modelKey]: value,
        },
      },
    })),

  getModelSettings: (modelId) => {
    const { modelSettings } = get();
    if (!modelId) return {};
    return modelSettings[modelId] ?? {};
  },

  getModelSetting: (modelId, modelKey) => {
    const { modelSettings } = get();
    if (!modelId) return null;
    return modelSettings[modelId]?.[modelKey] ?? null;
  },

  uploadedImages: [],

  addUploadedImages: (images: File[]) =>
    set((state) => {
      const existing = state.uploadedImages ?? [];
      const mapped: ChatboxUploadedImage[] = images.map((file) => {
        const previewUrl = URL.createObjectURL(file);
        return {
          id: generateImageId(),
          file,
          previewUrl,
          addedAt: Date.now(),
        };
      });

      return {
        uploadedImages: [...existing, ...mapped],
      };
    }),

  removeUploadedImage: (id) =>
    set((state) => {
      const filtered = state.uploadedImages.filter((img) => {
        const keep = img.id !== id;
        if (!keep) {
          URL.revokeObjectURL(img.previewUrl);
        }
        return keep;
      });

      return {
        uploadedImages: filtered,
      };
    }),

  clearUploadedImages: () =>
    set((state) => {
      state.uploadedImages.forEach((img) => {
        URL.revokeObjectURL(img.previewUrl);
      });
      return { uploadedImages: [] };
    }),
}));
