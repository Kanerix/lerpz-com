import { createQuery } from "@tanstack/svelte-query";
import { browser } from "$app/environment";
import type { ModelResponse as ApiModel } from "$lib/api/models";
import { listModels } from "$lib/api/models/models.js";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { queryKeys } from "$lib/query/keys.js";

export type ChatboxVariant = "chat" | "image" | "video";

/** The input/output modalities a model can support. */
export type Modality = "text" | "image" | "video";

export type ModelSetting = {
    key: string;
    name: string;
    icon: string;
    tooltip: string;
    defaultValue: string;
    values: { value: string | number; label: string }[];
};

export type Model = {
    label: string;
    value: string | null;
    description: string;
    family: string;
    provider: string;
    modalities: string[];
    features: string[];
    settings: ModelSetting[];
    reasoning: boolean;
};

/** The color scheme used when picking a theme-aware logo. */
export type ColorScheme = "light" | "dark";

/**
 * A family logo is either a single asset used in both themes, or a pair with a
 * dedicated variant per color scheme (e.g. a white logo for dark mode).
 */
type FamilyLogo = string | Record<ColorScheme, string>;

// Maps a model family to its logo(s) in `static/models/`.
const MODEL_FAMILY_LOGOS: Record<string, FamilyLogo> = {
    openai: {
        light: "/models/openai_light.svg",
        dark: "/models/openai_dark.svg",
    },
    anthropic: "/models/claude.svg",
    google: "/models/gemini.svg",
    xai: {
        light: "/models/grok_light.svg",
        dark: "/models/grok_dark.svg",
    },
};

export const FALLBACK_MODEL_LOGO = "/lerpz.svg";

export function modelFamilyLogo(
    family: string,
    scheme: ColorScheme = "light",
): string {
    const logo = MODEL_FAMILY_LOGOS[family.toLowerCase()];
    if (!logo) return FALLBACK_MODEL_LOGO;
    return typeof logo === "string" ? logo : logo[scheme];
}

export function filterModelsByModality(
    models: Model[],
    modality: Modality,
): Model[] {
    return models.filter((model) => model.modalities.includes(modality));
}

function toModel(model: ApiModel): Model {
    return {
        label: model.display_name,
        value: model.deployment_name,
        description: model.description ?? "",
        family: model.family ?? "",
        provider: model.provider ?? "",
        modalities: model.modalities ?? [],
        features: [],
        settings: [],
        reasoning: model.settings?.reasoning ?? false,
    };
}

export function createModels() {
    const accountKey = $derived(msalStore.accountKey);
    const query = createQuery(() => ({
        queryKey: queryKeys.models.list(accountKey),
        enabled: browser && accountKey !== null,
        queryFn: async ({ signal }: { signal: AbortSignal }) => {
            const response = await listModels({ signal });
            if (response.status !== 200) throw response.data;
            return response.data.map(toModel);
        },
    }));

    async function loadModels(_modality?: string) {
        await query.refetch();
    }

    return {
        get models() {
            return query.data ?? [];
        },
        get isLoading() {
            return query.isFetching;
        },
        loadModels,
    };
}
