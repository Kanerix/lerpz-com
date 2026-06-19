import { browser } from "$app/environment";

/** localStorage key used to persist the user's favorite models. */
const STORAGE_KEY = "lerpz:favorite-models";

function loadFavorites(): string[] {
    if (!browser) return [];
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        const parsed = raw ? JSON.parse(raw) : [];
        return Array.isArray(parsed)
            ? parsed.filter((v): v is string => typeof v === "string")
            : [];
    } catch {
        // Corrupt or inaccessible storage — start from an empty list.
        return [];
    }
}

function persistFavorites(values: string[]) {
    if (!browser) return;
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(values));
    } catch {
        // Ignore write failures (e.g. storage disabled or quota exceeded).
    }
}

/**
 * Persists the set of models the user has marked as favorite. The list is
 * stored by model value (deployment name) in `localStorage` so it survives
 * reloads.
 */
class ModelFavoritesStore {
    favorites = $state<string[]>(loadFavorites());

    isFavorite(value: string | null): boolean {
        if (!value) return false;
        return this.favorites.includes(value);
    }

    add(value: string | null) {
        if (!value || this.isFavorite(value)) return;
        this.favorites = [...this.favorites, value];
        persistFavorites(this.favorites);
    }

    remove(value: string | null) {
        if (!value) return;
        this.favorites = this.favorites.filter((v) => v !== value);
        persistFavorites(this.favorites);
    }

    toggle(value: string | null) {
        if (!value) return;
        if (this.isFavorite(value)) this.remove(value);
        else this.add(value);
    }
}

export const modelFavoritesStore = new ModelFavoritesStore();
