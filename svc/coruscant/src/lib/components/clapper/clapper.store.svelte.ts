export type AspectRatioOption = {
    value: string;
    label: string;
    /** Width/height used to render the little preview glyph. */
    w: number;
    h: number;
};

// The ratios most video models understand. `16:9` is the safe default for video.
export const WIDESCREEN_RATIO: AspectRatioOption = {
    value: "16:9",
    label: "Widescreen",
    w: 16,
    h: 9,
};

export const ASPECT_RATIOS: AspectRatioOption[] = [
    WIDESCREEN_RATIO,
    { value: "9:16", label: "Vertical", w: 9, h: 16 },
    { value: "1:1", label: "Square", w: 1, h: 1 },
    { value: "4:3", label: "Classic", w: 4, h: 3 },
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

// Clip lengths the models understand. Kept short since generation cost scales
// with duration.
export const SHORT_DURATION: DurationOption = { value: 4, label: "4 seconds" };

export const DURATIONS: DurationOption[] = [
    SHORT_DURATION,
    { value: 6, label: "6 seconds" },
    { value: 8, label: "8 seconds" },
];

export const DEFAULT_DURATION = SHORT_DURATION.value;

export function resolveDuration(value: number): DurationOption {
    return DURATIONS.find((d) => d.value === value) ?? SHORT_DURATION;
}

class ClapperStore {
    prompt = $state("");
    model = $state<string | null>(null);
    clapperAnchor = $state<HTMLElement | null>(null);
    aspectRatio = $state<string>(DEFAULT_ASPECT_RATIO);
    duration = $state<number>(DEFAULT_DURATION);

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    setModel(model: string | null) {
        this.model = model;
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
