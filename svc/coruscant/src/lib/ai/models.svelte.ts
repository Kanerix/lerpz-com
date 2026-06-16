import { listModels } from "$lib/api/models/models.js";
import type { Model as ApiModel } from "$lib/api/models/index.js";

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

function toModel(model: ApiModel): Model {
    return {
        label: model.display_name,
        value: model.deployment_name,
        description: model.description ?? "",
        modalities: [],
        features: [],
        settings: [],
    };
}

export function createModels() {
    let models = $state<Model[]>([]);
    let isLoading = $state(false);

    async function loadModels(_modality?: string) {
        isLoading = true;
        try {
            const response = await listModels();
            models =
                response.status === 200 ? response.data.map(toModel) : [];
        } catch (error) {
            console.error("Failed to load models", error);
            models = [];
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
