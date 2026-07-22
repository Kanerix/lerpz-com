import {
    MODEL_STORAGE_KEYS,
    loadStoredModel,
    storeModel,
} from "$lib/ai/model-storage.js";

export type AspectRatioOption = {
    value: string;
    label: string;
    /** Width/height used to render the little preview glyph. */
    w: number;
    h: number;
};

// Video generation only renders landscape (`16:9`) or portrait (`9:16`); any
// other ratio is coerced to portrait server-side, so we don't offer them.
// `16:9` is the safe default for video.
export const WIDESCREEN_RATIO: AspectRatioOption = {
    value: "16:9",
    label: "Widescreen",
    w: 16,
    h: 9,
};

export const ASPECT_RATIOS: AspectRatioOption[] = [
    WIDESCREEN_RATIO,
    { value: "9:16", label: "Vertical", w: 9, h: 16 },
];

export const DEFAULT_ASPECT_RATIO = WIDESCREEN_RATIO.value;

export function resolveAspectRatio(value: string): AspectRatioOption {
    return ASPECT_RATIOS.find((r) => r.value === value) ?? WIDESCREEN_RATIO;
}

export type DurationOption = {
    /** Duration in seconds. */
    value: number;
    label: string;
};

// Clip lengths the models understand. Video generation clamps requests to the
// 4-8 second range and defaults to the longest, so those are the values we
// expose.
export const DURATIONS: DurationOption[] = [
    { value: 4, label: "4 seconds" },
    { value: 6, label: "6 seconds" },
    { value: 8, label: "8 seconds" },
];

export const DEFAULT_DURATION = 8;

export function resolveDuration(value: number): DurationOption {
    return (
        DURATIONS.find((d) => d.value === value) ??
        DURATIONS[DURATIONS.length - 1]
    );
}

class ClapperStore {
    prompt = $state("");
    model = $state<string | null>(loadStoredModel(MODEL_STORAGE_KEYS.video));
    clapperAnchor = $state<HTMLElement | null>(null);
    aspectRatio = $state<string>(DEFAULT_ASPECT_RATIO);
    duration = $state<number>(DEFAULT_DURATION);

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    setModel(model: string | null) {
        this.model = model;
        storeModel(MODEL_STORAGE_KEYS.video, model);
    }
    setClapperAnchor(el: HTMLElement | null) {
        this.clapperAnchor = el;
    }
    setAspectRatio(ratio: string) {
        this.aspectRatio = ratio;
    }
    setDuration(duration: number) {
        this.duration = duration;
    }

    reset() {
        this.prompt = "";
        this.aspectRatio = DEFAULT_ASPECT_RATIO;
        this.duration = DEFAULT_DURATION;
    }
}

export const clapperStore = new ClapperStore();
