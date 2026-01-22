import { create } from "zustand";

type ChatboxState = {
  prompt: string | null;
  setPrompt: (prompt: string | null) => void;
};

export const useChatboxStore = create<ChatboxState>((set) => ({
  prompt: null,
  setPrompt: (prompt) => set({ prompt }),
}));
