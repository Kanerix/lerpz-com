import { create } from "zustand";

export interface ChatboxUploadedImage {
  id: string;
  file: File;
  previewUrl: string;
  addedAt: number;
}

export interface ChatboxState {
  prompt: string;
  setPrompt: (prompt: string) => void;

  model?: string;
  setModel: (model?: string) => void;

  autoAnalyze: boolean;
  setAutoAnalyze: (autoAnalyze: boolean) => void;

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

export type ImageQuality = "auto" | "low" | "medium" | "high";
export type ImageAmount = "1" | "2" | "3" | "4" | "5" | "6";
export type ImageFiletype = "png" | "webp" | "jpeg";
export type ImageBackground = "auto" | "transparent" | "opaque";

export interface ChatboxImageSettingsState {
  model: string | null;
  setModel: (value: string | null) => void;

  autoAnalyze: boolean;
  setAutoAnalyze: (value: boolean) => void;
  toggleAutoAnalyze: () => void;

  quality: ImageQuality;
  setQuality: (value: ImageQuality) => void;

  amount: ImageAmount;
  setAmount: (value: ImageAmount) => void;

  filetype: ImageFiletype;
  setFiletype: (value: ImageFiletype) => void;

  background: ImageBackground;
  setBackground: (value: ImageBackground) => void;

  reset: () => void;
}

const defaultImageSettings: Omit<
  ChatboxImageSettingsState,
  | "setModel"
  | "setAutoAnalyze"
  | "toggleAutoAnalyze"
  | "setQuality"
  | "setAmount"
  | "setFiletype"
  | "setBackground"
  | "reset"
> = {
  model: null,
  autoAnalyze: true,
  quality: "auto",
  amount: "1",
  filetype: "png",
  background: "auto",
};

const CHATBOX_IMAGE_SETTINGS_STORAGE_KEY = "chatbox_image_settings";

function isBrowserEnv(): boolean {
  return (
    typeof window !== "undefined" && typeof window.localStorage !== "undefined"
  );
}

function loadImageSettingsFromStorage(): Partial<ChatboxImageSettingsState> | null {
  if (!isBrowserEnv()) return null;

  try {
    const raw = window.localStorage.getItem(CHATBOX_IMAGE_SETTINGS_STORAGE_KEY);
    if (!raw) return null;

    const parsed = JSON.parse(raw) as Partial<ChatboxImageSettingsState>;

    const merged: ChatboxImageSettingsState = {
      ...defaultImageSettings,
      model: parsed.model || null,
      autoAnalyze:
        typeof parsed.autoAnalyze === "boolean"
          ? parsed.autoAnalyze
          : defaultImageSettings.autoAnalyze,
      quality:
        parsed.quality &&
        ["auto", "low", "high"].includes(parsed.quality as string)
          ? (parsed.quality as ImageQuality)
          : defaultImageSettings.quality,
      amount:
        parsed.amount &&
        ["1", "2", "3", "4", "5", "6"].includes(parsed.amount as string)
          ? (parsed.amount as ImageAmount)
          : defaultImageSettings.amount,
      filetype:
        parsed.filetype &&
        ["png", "webp", "jpeg"].includes(parsed.filetype as string)
          ? (parsed.filetype as ImageFiletype)
          : defaultImageSettings.filetype,
      background:
        parsed.background &&
        ["auto", "transparent", "opaque"].includes(parsed.background as string)
          ? (parsed.background as ImageBackground)
          : defaultImageSettings.background,
      setModel: () => {},
      setAutoAnalyze: () => {},
      toggleAutoAnalyze: () => {},
      setQuality: () => {},
      setAmount: () => {},
      setFiletype: () => {},
      setBackground: () => {},
      reset: () => {},
    };

    return merged;
  } catch {
    return null;
  }
}

function persistImageSettingsToStorage(state: ChatboxImageSettingsState): void {
  if (!isBrowserEnv()) return;

  const { model, autoAnalyze, quality, amount, filetype, background } = state;
  try {
    const payload = JSON.stringify({
      model,
      autoAnalyze,
      quality,
      amount,
      filetype,
      background,
    });
    window.localStorage.setItem(CHATBOX_IMAGE_SETTINGS_STORAGE_KEY, payload);
  } catch {
    // ignore storage errors
  }
}

export const useChatboxImageStore = create<ChatboxImageSettingsState>((set) => {
  const initial = loadImageSettingsFromStorage() ?? defaultImageSettings;

  const withPersistence = (partial: Partial<ChatboxImageSettingsState>) => {
    set((state) => {
      const next = { ...state, ...partial };
      persistImageSettingsToStorage(next);
      return next;
    });
  };

  return {
    ...(initial as ChatboxImageSettingsState),

    setModel: (value) =>
      withPersistence({
        model: value,
      }),

    setAutoAnalyze: (value) =>
      withPersistence({
        autoAnalyze: value,
      }),

    toggleAutoAnalyze: () =>
      set((state) => {
        const next: ChatboxImageSettingsState = {
          ...state,
          autoAnalyze: !state.autoAnalyze,
        };
        persistImageSettingsToStorage(next);
        return next;
      }),

    setQuality: (value) =>
      withPersistence({
        quality: value,
      }),

    setAmount: (value) =>
      withPersistence({
        amount: value,
      }),

    setFiletype: (value) =>
      withPersistence({
        filetype: value,
      }),

    setBackground: (value) =>
      withPersistence({
        background: value,
      }),

    reset: () => withPersistence(defaultImageSettings),
  };
});
