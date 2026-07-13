import { getContext, setContext } from "svelte";
import type { Model } from "$lib/ai/models.svelte.js";

export type EaselSubmitArgs = {
    prompt: string;
    model: string | null;
    aspectRatio: string;
    count: number;
};

export type EaselContextValue = {
    readonly models: Model[];
    readonly isModelsLoading: boolean;
    submit: () => Promise<void>;
    readonly isSubmitPending: boolean;
    enhance?: (prompt: string) => Promise<string>;
    readonly isEnhancePending: boolean;
    readonly isPending: boolean;
};

const EASEL_KEY = Symbol("easel");

export function setEaselContext(ctx: EaselContextValue) {
    setContext(EASEL_KEY, ctx);
}

export function getEaselContext(): EaselContextValue {
    const ctx = getContext<EaselContextValue>(EASEL_KEY);
    if (!ctx) throw new Error("getEaselContext must be used inside <Easel>");
    return ctx;
}
