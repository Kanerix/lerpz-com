import {
    MODEL_STORAGE_KEYS,
    loadStoredModel,
    storeModel,
} from "$lib/ai/model-storage.js";

export type AspectRatioOption = {
    value: string;
    label: string;
    w: number;
    h: number;
};

export const SQUARE_RATIO: AspectRatioOption = {
    value: "1:1",
    label: "Square",
    w: 1,
    h: 1,
};

export const ASPECT_RATIOS: AspectRatioOption[] = [
    SQUARE_RATIO,
    { value: "3:4", label: "Portrait", w: 3, h: 4 },
    { value: "4:3", label: "Landscape", w: 4, h: 3 },
    { value: "9:16", label: "Tall", w: 9, h: 16 },
    { value: "16:9", label: "Wide", w: 16, h: 9 },
];

export const DEFAULT_ASPECT_RATIO = SQUARE_RATIO.value;

export function resolveAspectRatio(value: string): AspectRatioOption {
    return ASPECT_RATIOS.find((r) => r.value === value) ?? SQUARE_RATIO;
}

// How many images a single prompt may fan out into.
export const MIN_IMAGE_COUNT = 1;
export const MAX_IMAGE_COUNT = 4;

class EaselStore {
    prompt = $state("");
    model = $state<string | null>(loadStoredModel(MODEL_STORAGE_KEYS.image));
    easelAnchor = $state<HTMLElement | null>(null);
    aspectRatio = $state<string>(DEFAULT_ASPECT_RATIO);
    count = $state<number>(1);

    setPrompt(prompt: string) {
        this.prompt = prompt;
    }
    setModel(model: string | null) {
        this.model = model;
        storeModel(MODEL_STORAGE_KEYS.image, model);
    }
    setEaselAnchor(el: HTMLElement | null) {
        this.easelAnchor = el;
    }
    setAspectRatio(ratio: string) {
        this.aspectRatio = ratio;
    }
    setCount(count: number) {
        this.count = Math.min(
            MAX_IMAGE_COUNT,
            Math.max(MIN_IMAGE_COUNT, Math.round(count)),
        );
    }

    reset() {
        this.prompt = "";
        this.aspectRatio = DEFAULT_ASPECT_RATIO;
        this.count = 1;
    }
}

export const easelStore = new EaselStore();
