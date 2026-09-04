<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import {
    Popover,
    PopoverContent,
    PopoverPositioner,
    PopoverTrigger,
} from "@lerpz/ui/components/popover";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Separator } from "@lerpz/ui/components/separator";
import { cn } from "@lerpz/ui/lib/utils";
import { modelFavoritesStore } from "$lib/ai/model-favorites.svelte.js";
import { modelFamilyLogoForTheme } from "$lib/ai/model-logo.svelte.js";
import type { Model } from "$lib/ai/models.svelte.js";
import { DEFAULT_REASONING_LEVEL, REASONING_LEVELS } from "./reasoning.js";

let {
    models = [],
    isModelsLoading = false,
    value = null,
    onSelect,
    getReasoningLevel,
    onReasoningChange,
    getAnchorRect,
}: {
    models?: Model[];
    isModelsLoading?: boolean;
    value?: string | null;
    onSelect: (value: string | null) => void;
    getReasoningLevel?: (model: Model) => string | null;
    onReasoningChange?: (model: Model, level: string) => void;
    getAnchorRect?: () => DOMRect | null;
} = $props();

const sortedModels = $derived<Model[]>(
    [...models].sort(
        (a, b) =>
            a.family.localeCompare(b.family) || a.label.localeCompare(b.label),
    ),
);

let open = $state(false);
// Which screen the popover is showing: the model list or a single model's page.
let view = $state<"list" | "detail">("list");
// Model highlighted by keyboard navigation while on the list.
let previewValue = $state<string | null>(null);
// Model whose detail page is currently open.
let detailValue = $state<string | null>(null);

const selectedModel = $derived<Model | null>(
    sortedModels.find((m) => m.value === value) ?? null,
);

const favoriteModels = $derived<Model[]>(
    sortedModels.filter((m) => modelFavoritesStore.isFavorite(m.value)),
);

const nonFavoriteModels = $derived<Model[]>(
    sortedModels.filter((m) => !modelFavoritesStore.isFavorite(m.value)),
);

// Models in the order they appear in the list (favorites first).
const visibleModels = $derived<Model[]>([
    ...favoriteModels,
    ...nonFavoriteModels,
]);

const detailModel = $derived<Model | null>(
    sortedModels.find((m) => m.value === detailValue) ?? null,
);

const positioning = $derived({
    placement: "top" as const,
    gutter: 8,
    ...(getAnchorRect ? { getAnchorRect } : {}),
});

function handleOpenChange(details: { open: boolean }) {
    open = details.open;
    if (details.open) {
        view = "list";
        detailValue = null;
        previewValue = value;
    }
}

function openDetail(modelValue: string | null) {
    if (!modelValue) return;
    detailValue = modelValue;
    view = "detail";
}

function backToList() {
    view = "list";
    previewValue = detailValue ?? value;
}

function selectModel(modelValue: string | null) {
    onSelect(modelValue);
    open = false;
}

const reasoningLevel = $derived<string | null>(
    detailModel?.reasoning && getReasoningLevel
        ? (getReasoningLevel(detailModel) ?? DEFAULT_REASONING_LEVEL)
        : null,
);

function setReasoningLevel(level: string) {
    if (!detailModel?.reasoning) return;
    onReasoningChange?.(detailModel, level);
}

function handleListKeydown(e: KeyboardEvent) {
    const list = visibleModels;
    if (list.length === 0) return;
    const currentIndex = list.findIndex((m) => m.value === previewValue);
    if (e.key === "ArrowDown") {
        e.preventDefault();
        const next = list[(currentIndex + 1 + list.length) % list.length];
        if (next) previewValue = next.value;
    } else if (e.key === "ArrowUp") {
        e.preventDefault();
        const prev = list[(currentIndex - 1 + list.length) % list.length];
        if (prev) previewValue = prev.value;
    } else if (e.key === "Enter" && previewValue) {
        e.preventDefault();
        openDetail(previewValue);
    }
}
</script>

{#snippet modelRow(model: Model)}
  {@const isSelected = model.value === value}
  {@const isPreview = model.value === previewValue}
  {@const isFavorite = modelFavoritesStore.isFavorite(model.value)}
  <li role="option" aria-selected={isSelected}>
    <div
      class={cn(
        "flex w-full items-center gap-1 rounded-md pr-1 transition-colors",
        "hover:bg-muted dark:hover:bg-muted/50",
        isPreview && "bg-muted dark:bg-muted/50",
      )}
    >
      <button
        type="button"
        class="flex min-w-0 grow cursor-pointer items-center gap-2 rounded-md py-2 pl-2.5 text-left text-sm"
        onmouseenter={() => (previewValue = model.value)}
        onfocus={() => (previewValue = model.value)}
        onclick={() => openDetail(model.value)}
      >
        <img
          src={modelFamilyLogoForTheme(model.family)}
          alt=""
          class="size-4 shrink-0 object-contain"
        />
        <span class="min-w-0 grow truncate">{model.label}</span>
        {#if isSelected}
          <Icon icon="fa6-solid:check" class="size-4 shrink-0 text-primary" />
        {/if}
        <Icon
          icon="fa6-solid:angle-right"
          class="size-3.5 shrink-0 opacity-40"
        />
      </button>
      <button
        type="button"
        class={cn(
          "flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md transition-colors",
          "text-muted-foreground hover:bg-background/80 hover:text-foreground",
          isFavorite && "text-amber-500 hover:text-amber-500",
        )}
        aria-pressed={isFavorite}
        aria-label={isFavorite ? "Remove from favorites" : "Add to favorites"}
        onclick={() => modelFavoritesStore.toggle(model.value)}
      >
        <Icon
          icon={isFavorite ? "fa6-solid:star" : "fa6-regular:star"}
          class="size-3.5"
        />
      </button>
    </div>
  </li>
{/snippet}

<Popover {open} onOpenChange={handleOpenChange} {positioning}>
  <PopoverTrigger
    class={cn(
      "inline-flex h-9 items-center gap-1.5 rounded-4xl px-3 text-sm",
      "border border-transparent bg-transparent transition-colors",
      "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
      "data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground",
    )}
    aria-label="Select model"
  >
    {#if isModelsLoading}
      <Icon icon="fa6-solid:spinner" class="animate-spin" />
    {:else if selectedModel}
      <img
        src={modelFamilyLogoForTheme(selectedModel.family)}
        alt=""
        class="size-4 shrink-0 object-contain"
      />
    {:else}
      <Icon icon="fa6-solid:robot" />
    {/if}
    <span class="max-w-40 truncate">
      {selectedModel?.label ?? "Select model"}
    </span>
    <Icon icon="fa6-solid:angle-up" class="size-4 opacity-60" />
  </PopoverTrigger>

  <PopoverPositioner>
    <PopoverContent
      class="flex h-2xl w-full max-w-lg min-w-3xs flex-col overflow-hidden rounded-xl p-0 text-left"
    >
      {#if view === "list"}
        <!-- Model list -->
        <div
          class="px-3 pt-3 pb-1.5 text-xs font-medium text-muted-foreground"
        >
          Models
        </div>
        <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
        <ul
          class="flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto p-1.5"
          tabindex="0"
          role="listbox"
          aria-label="Available models"
          onkeydown={handleListKeydown}
        >
          {#if isModelsLoading}
            <li class="px-3 py-2 text-sm text-muted-foreground">
              <Icon icon="fa6-solid:spinner" class="mr-1.5 inline animate-spin" />
              Loading models…
            </li>
          {:else if sortedModels.length === 0}
            <li class="px-3 py-2 text-sm text-muted-foreground">
              No models available
            </li>
          {:else}
            {#if favoriteModels.length > 0}
              <li
                aria-hidden="true"
                class="px-2.5 pt-1 pb-1 text-xs font-medium text-muted-foreground"
              >
                Favorites
              </li>
              {#each favoriteModels as model (`fav-${model.value}`)}
                {@render modelRow(model)}
              {/each}
              <li
                aria-hidden="true"
                class="px-2.5 pt-2 pb-1 text-xs font-medium text-muted-foreground"
              >
                All models
              </li>
            {/if}
            {#each nonFavoriteModels as model (model.value)}
              {@render modelRow(model)}
            {/each}
          {/if}
        </ul>
      {:else if detailModel}
        {@const isDetailFavorite = modelFavoritesStore.isFavorite(
          detailModel.value,
        )}
        <!-- Single model page -->
        <div class="flex items-center gap-2 border-b p-2">
          <button
            type="button"
            class="flex cursor-pointer items-center gap-1.5 rounded-md px-2 py-1.5 text-sm font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            onclick={backToList}
          >
            <Icon icon="fa6-solid:angle-left" class="size-3.5" />
            Models
          </button>
          <button
            type="button"
            class={cn(
              "ml-auto flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-md transition-colors",
              "text-muted-foreground hover:bg-muted hover:text-foreground",
              isDetailFavorite && "text-amber-500 hover:text-amber-500",
            )}
            aria-pressed={isDetailFavorite}
            aria-label={isDetailFavorite
              ? "Remove from favorites"
              : "Add to favorites"}
            onclick={() => modelFavoritesStore.toggle(detailModel.value)}
          >
            <Icon
              icon={isDetailFavorite ? "fa6-solid:star" : "fa6-regular:star"}
              class="size-4"
            />
          </button>
        </div>

        <ScrollArea orientation="vertical" class="flex-1">
          <div class="flex flex-col gap-4 p-4">
            <div class="flex items-start gap-3">
              <img
                src={modelFamilyLogoForTheme(detailModel.family)}
                alt=""
                class="size-9 shrink-0 object-contain"
              />
              <div class="flex min-w-0 grow flex-col gap-1">
                <h3 class="text-base leading-tight font-semibold">
                  {detailModel.label}
                </h3>
                {#if detailModel.provider || detailModel.family}
                  <p class="text-xs text-muted-foreground">
                    {[detailModel.provider, detailModel.family]
                      .filter(Boolean)
                      .join(" · ")}
                  </p>
                {/if}
              </div>
            </div>

            {#if detailModel.description}
              <p class="text-sm leading-relaxed text-muted-foreground">
                {detailModel.description}
              </p>
            {/if}

            {#if detailModel.modalities.length > 0 || detailModel.features.length > 0}
              <div class="flex flex-col gap-2">
                <span class="text-xs font-medium text-muted-foreground">
                  Tags
                </span>
                <div class="flex flex-wrap gap-1.5">
                  {#each detailModel.modalities as modality (modality)}
                    <Badge variant="secondary">{modality}</Badge>
                  {/each}
                  {#each detailModel.features as feature (feature)}
                    <Badge variant="outline">{feature}</Badge>
                  {/each}
                </div>
              </div>
            {/if}

            <Separator />

            <div class="flex flex-col gap-3">
              <span class="text-xs font-medium text-muted-foreground">
                Settings
              </span>
              {#if detailModel.reasoning && onReasoningChange}
                <div class="flex flex-col gap-2">
                  <div class="flex flex-col">
                    <span class="text-sm font-medium">Reasoning</span>
                    <span class="text-xs text-muted-foreground">
                      Let the model think through problems step by step.
                    </span>
                  </div>
                  <div
                    class="inline-flex gap-0.5 rounded-md border p-0.5"
                    role="group"
                    aria-label="Reasoning level"
                  >
                    {#each REASONING_LEVELS as level (level.value)}
                      <button
                        type="button"
                        aria-pressed={reasoningLevel === level.value}
                        onclick={() => setReasoningLevel(level.value)}
                        class={cn(
                          "flex-1 cursor-pointer rounded-sm px-2.5 py-1 text-xs font-medium transition-colors",
                          reasoningLevel === level.value
                            ? "bg-primary text-primary-foreground"
                            : "text-muted-foreground hover:bg-muted",
                        )}
                      >
                        {level.label}
                      </button>
                    {/each}
                  </div>
                </div>
              {:else}
                <p class="text-xs text-muted-foreground">
                  This model has no configurable settings.
                </p>
              {/if}
            </div>
          </div>
        </ScrollArea>

        <div class="border-t p-3">
          <button
            type="button"
            class={cn(
              "flex w-full cursor-pointer items-center justify-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium transition-colors",
              detailModel.value === value
                ? "bg-muted text-muted-foreground"
                : "bg-primary text-primary-foreground hover:bg-primary/90",
            )}
            disabled={detailModel.value === value}
            onclick={() => selectModel(detailModel.value)}
          >
            {#if detailModel.value === value}
              <Icon icon="fa6-solid:check" class="size-4" />
              Selected
            {:else}
              Use this model
            {/if}
          </button>
        </div>
      {:else}
        <div
          class="flex flex-1 items-center justify-center p-4 text-sm text-muted-foreground"
        >
          Select a model to see details
        </div>
      {/if}
    </PopoverContent>
  </PopoverPositioner>
</Popover>
