import { toast } from "svelte-sonner";
import {
    enhanceChatPrompt,
    enhanceImagePrompt,
    enhanceVideoPrompt,
} from "$lib/api/enhance/enhance.js";
import type { EnhanceRequest } from "$lib/api/models/index.js";

type EnhanceResult = { status: number; data: unknown };
type EnhanceFn = (
    enhanceRequest: EnhanceRequest,
    options?: RequestInit,
) => Promise<EnhanceResult>;

/** How long the "undo enhancement" toast stays on screen (milliseconds). */
const UNDO_TOAST_DURATION_MS = 10_000;

/**
 * Shows a toast that lets the user restore their original prompt after an
 * enhancement. It lingers for {@link UNDO_TOAST_DURATION_MS} so there is time to
 * read it and change their mind before it disappears.
 */
export function notifyPromptEnhanced(restore: () => void): void {
    toast.success("Prompt enhanced", {
        description:
            "Not what you had in mind? Undo to restore your original prompt.",
        duration: UNDO_TOAST_DURATION_MS,
        action: {
            label: "Undo",
            onClick: restore,
        },
    });
}

/**
 * Sends a prompt to an enhancement endpoint and returns the improved prompt.
 *
 * Enhancement is best-effort: if the request fails or the model returns nothing
 * usable, the original prompt is returned unchanged and the failure is surfaced
 * via a toast, so the composer never ends up empty or stuck.
 */
async function runEnhance(
    fn: EnhanceFn,
    kind: string,
    prompt: string,
    model?: string | null,
): Promise<string> {
    const trimmed = prompt.trim();
    if (!trimmed) return prompt;

    try {
        const response = await fn({ prompt: trimmed, model: model ?? null });
        const data = response?.data as { prompt?: string } | undefined;
        const enhanced = data?.prompt;
        if (typeof enhanced === "string" && enhanced.trim()) {
            return enhanced;
        }
        throw new Error("The model did not return an enhanced prompt.");
    } catch (err) {
        toast.error(`Couldn't enhance the ${kind} prompt`, {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
        return prompt;
    }
}

/** Enhances a conversational chat prompt. */
export function enhanceChat(
    prompt: string,
    model?: string | null,
): Promise<string> {
    return runEnhance(enhanceChatPrompt, "chat", prompt, model);
}

/** Enhances a text-to-image generation prompt. */
export function enhanceImage(
    prompt: string,
    model?: string | null,
): Promise<string> {
    return runEnhance(enhanceImagePrompt, "image", prompt, model);
}

/** Enhances a text-to-video generation prompt. */
export function enhanceVideo(
    prompt: string,
    model?: string | null,
): Promise<string> {
    return runEnhance(enhanceVideoPrompt, "video", prompt, model);
}
