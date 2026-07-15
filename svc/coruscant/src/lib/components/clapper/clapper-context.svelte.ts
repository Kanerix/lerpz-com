import { getContext, setContext } from "svelte";
import type { Model } from "$lib/ai/models.svelte.js";

export type ClapperSubmitArgs = {
    prompt: string;
    model: string | null;
    aspectRatio: string;
    /** Clip length in seconds. */
    duration: number;
};

export type ClapperContextValue = {
    readonly models: Model[];
    readonly isModelsLoading: boolean;
    submit: () => Promise<void>;
    readonly isSubmitPending: boolean;
    enhance?: (prompt: string) => Promise<string>;
    readonly isEnhancePending: boolean;
    readonly isPending: boolean;
};

const CLAPPER_KEY = Symbol("clapper");

export function setClapperContext(ctx: ClapperContextValue) {
    setContext(CLAPPER_KEY, ctx);
}

export function getClapperContext(): ClapperContextValue {
    const ctx = getContext<ClapperContextValue>(CLAPPER_KEY);
    if (!ctx)
        throw new Error("getClapperContext must be used inside <Clapper>");
    return ctx;
}
