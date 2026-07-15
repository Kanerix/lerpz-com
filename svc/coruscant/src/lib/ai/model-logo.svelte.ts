import { mode } from "mode-watcher";
import { fromStore } from "svelte/store";

import { modelFamilyLogo } from "./models.svelte.js";

// `mode` from mode-watcher is a Svelte store; `fromStore` exposes it as a
// rune-friendly reactive value so this helper can be used from `.svelte.ts`.
const modeState = fromStore(mode);

/**
 * Resolves a model family's logo for the currently active theme.
 *
 * Reading this inside a component template or `$derived` tracks the theme, so
 * the logo swaps automatically between the light and dark variants. Returns
 * `null` when no family is provided (e.g. an in-flight message that has no
 * model yet), letting callers fall back to a placeholder.
 */
export function modelFamilyLogoForTheme(
    family: string | null | undefined,
): string | null {
    if (family == null) return null;
    return modelFamilyLogo(family, modeState.current === "dark" ? "dark" : "light");
}
