import { browser } from "$app/environment";

/**
 * Small helpers for persisting the model a user has selected in a given surface
 * (chat, image, video) so it survives page reloads. The value stored is the
 * model's `value` (deployment name).
 */

/** localStorage keys for each model-selecting surface. */
export const MODEL_STORAGE_KEYS = {
    chat: "lerpz:chat-model",
    image: "lerpz:image-model",
    video: "lerpz:video-model",
} as const;

export function loadStoredModel(key: string): string | null {
    if (!browser) return null;
    try {
        return localStorage.getItem(key);
    } catch {
        return null;
    }
}

export function storeModel(key: string, value: string | null): void {
    if (!browser) return;
    try {
        if (value === null) localStorage.removeItem(key);
        else localStorage.setItem(key, value);
    } catch {
        // Ignore write failures (e.g. storage disabled or quota exceeded).
    }
}
