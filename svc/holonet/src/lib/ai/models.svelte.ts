import { fakeDelay } from "$lib/utils/delay.js";

export type ChatboxVariant = "chat" | "image" | "video";

export type ModelSetting = {
    key: string;
    name: string;
    icon: string; // iconify icon name e.g. "mdi:photo-size-select-large"
    tooltip: string;
    defaultValue: string;
    values: { value: string | number; label: string }[];
};

export type Model = {
    label: string;
    value: string | null;
    description: string;
    modalities: string[];
    features: string[];
    settings: ModelSetting[];
};

const currentModels: Model[] = [
    {
        label: "GPT Image 1",
        value: "gpt-image-1",
        description:
            "OpenAI's flagship image generation model with support for both generating and editing images.",
        modalities: ["image"],
        features: ["generate", "edit"],
        settings: [
            {
                name: "Quality",
                key: "quality",
                icon: "mdi:photo-size-select-large",
                tooltip: "Quality of the image",
                defaultValue: "medium",
                values: [
                    { value: "low", label: "Low" },
                    { value: "medium", label: "Medium" },
                    { value: "high", label: "High" },
                ],
            },
            {
                name: "Dimensions",
                key: "size",
                icon: "mdi:crop-square",
                tooltip: "Dimensions of the image",
                defaultValue: "1024x1024",
                values: [
                    { value: "1024x1024", label: "1024x1024" },
                    { value: "1536x1024", label: "1536x1024" },
                    { value: "1024x1536", label: "1024x1536" },
                ],
            },
            {
                name: "Amount",
                key: "n",
                icon: "mdi:collections",
                tooltip: "Amount of images",
                defaultValue: "1",
                values: [1, 2, 3, 4, 5, 6].map((v) => ({
                    value: v,
                    label: String(v),
                })),
            },
            {
                name: "Filetype",
                key: "output_format",
                icon: "mdi:insert-drive-file",
                tooltip: "Type of image",
                defaultValue: "png",
                values: [
                    { value: "png", label: "PNG" },
                    { value: "jpeg", label: "JPG" },
                ],
            },
            {
                name: "Background",
                key: "background",
                icon: "mdi:insert-drive-file",
                tooltip: "Background of the image",
                defaultValue: "auto",
                values: [
                    { value: "auto", label: "Auto" },
                    { value: "transparent", label: "Transparent" },
                    { value: "opaque", label: "Opaque" },
                ],
            },
        ],
    },
    {
        label: "Nano Banana",
        value: "gemini-2.5-flash-image",
        description:
            "Powered by Gemini 2.5 Flash, this model offers fast image generation with flexible aspect ratio options.",
        modalities: ["image"],
        features: ["generate"],
        settings: [
            {
                name: "Aspect Ratio",
                key: "aspect_ratio",
                icon: "mdi:crop-square",
                tooltip: "Aspect ratio of image",
                defaultValue: "1:1",
                values: ["1:1", "3:2", "2:3", "4:3", "3:4", "9:16", "16:9"].map(
                    (v) => ({ value: v, label: v }),
                ),
            },
        ],
    },
];

export function createModels() {
    let models = $state<Model[]>([]);
    let isLoading = $state(false);

    async function loadModels(modality?: string) {
        isLoading = true;
        try {
            await fakeDelay(1000);
            models = modality
                ? currentModels.filter((m) => m.modalities.includes(modality))
                : currentModels;
        } finally {
            isLoading = false;
        }
    }

    return {
        get models() {
            return models;
        },
        get isLoading() {
            return isLoading;
        },
        loadModels,
    };
}
