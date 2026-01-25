import { create } from "zustand";

export interface ChatboxUploadedImage {
  id: string;
  file: File;
  previewUrl: string;
  addedAt: number;
}

export type ChatboxModelSettings = Record<string, string | null>;

export interface ChatboxState {
  prompt: string;
  setPrompt: (prompt: string) => void;

  model?: string;
  setModel: (model?: string) => void;

  autoAnalyze: boolean;
  setAutoAnalyze: (autoAnalyze: boolean) => void;

  modelSettings: ChatboxModelSettings;
  setModelSetting: (modelKey: string, value: string | null) => void;

  uploadedImages: ChatboxUploadedImage[];
  addUploadedImages: (images: File[]) => void;
  removeUploadedImage: (id: string) => void;
  clearUploadedImages: () => void;
}

function generateImageId(prefix: string = "img"): string {
  return `${prefix}_${Math.random().toString(36).slice(2)}_${Date.now()}`;
}

export const useChatboxStore = create<ChatboxState>((set) => ({
  prompt: "",
  setPrompt: (prompt) => {
    return set({ prompt });
  },

  model: undefined,
  setModel: (model) => {
    return set({ model });
  },

  autoAnalyze: true,
  setAutoAnalyze: (autoAnalyze) => {
    return set({ autoAnalyze });
  },

  modelSettings: {},
  setModelSetting: (modelKey, value) =>
    set((state) => ({
      modelSettings: {
        ...state.modelSettings,
        [modelKey]: value,
      },
    })),

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
        const isEqual = img.id !== id;
        if (isEqual) URL.revokeObjectURL(img.previewUrl);
        return isEqual;
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
