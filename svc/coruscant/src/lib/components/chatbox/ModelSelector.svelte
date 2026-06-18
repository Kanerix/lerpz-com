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
import { type Model, modelFamilyLogo } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import { getChatboxContext } from "./chatbox-context.svelte.js";

const chatbox = getChatboxContext();

const REASONING_KEY = "reasoning";
const REASONING_DISABLED = "none";

const sortedModels = $derived<Model[]>(
    [...chatbox.models].sort(
        (a, b) =>
            a.family.localeCompare(b.family) || a.label.localeCompare(b.label),
    ),
);

let open = $state(false);
let previewValue = $state<string | null>(null);

const selectedModel = $derived<Model | null>(
    sortedModels.find((m) => m.value === chatboxStore.model) ?? null,
);

const previewModel = $derived<Model | null>(
    sortedModels.find((m) => m.value === previewValue) ??
        selectedModel ??
        sortedModels[0] ??
        null,
);

function handleOpenChange(details: { open: boolean }) {
    open = details.open;
    if (details.open) previewValue = chatboxStore.model;
}

function selectModel(value: string | null) {
    chatboxStore.setModel(value);
    open = false;
}

const reasoningEnabled = $derived(
    previewModel?.reasoning
        ? chatboxStore.getModelSetting(
              previewModel.value ?? undefined,
              REASONING_KEY,
          ) !== REASONING_DISABLED
        : false,
);

function toggleReasoning() {
    if (!previewModel?.value || !previewModel.reasoning) return;
    chatboxStore.setModelSetting(
        previewModel.value,
        REASONING_KEY,
        reasoningEnabled ? REASONING_DISABLED : null,
    );
}

function handleListKeydown(e: KeyboardEvent) {
    const models = sortedModels;
    if (models.length === 0) return;
    const currentIndex = models.findIndex((m) => m.value === previewValue);
    if (e.key === "ArrowDown") {
        e.preventDefault();
        const next = models[(currentIndex + 1 + models.length) % models.length];
        if (next) previewValue = next.value;
    } else if (e.key === "ArrowUp") {
        e.preventDefault();
        const prev = models[(currentIndex - 1 + models.length) % models.length];
        if (prev) previewValue = prev.value;
    } else if (e.key === "Enter" && previewModel) {
        e.preventDefault();
        selectModel(previewModel.value);
    }
}
</script>

<Popover
  {open}
  onOpenChange={handleOpenChange}
  positioning={{
    placement: "top",
    gutter: 8,
    sameWidth: true,
    getAnchorRect: () =>
      chatboxStore.chatboxAnchor?.getBoundingClientRect() ?? null,
  }}
>
  <PopoverTrigger
    class={cn(
      "inline-flex h-9 items-center gap-1.5 rounded-4xl px-3 text-sm",
      "border border-transparent bg-transparent transition-colors",
      "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
      "data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground",
    )}
    aria-label="Select model"
  >
    {#if chatbox.isModelsLoading}
      <Icon icon="fa6-solid:spinner" class="animate-spin" />
    {:else if selectedModel}
      <img
        src={modelFamilyLogo(selectedModel.family)}
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
      class="flex h-2xl w-full max-w-4xl overflow-hidden p-0 text-left"
    >
      <!-- Model list -->
      <div class="flex w-fit max-w-3xl shrink-0 flex-col border-r">
        <div class="px-3 pt-3 pb-1.5 text-xs font-medium text-muted-foreground">
          Models
        </div>
        <ul aria-hidden="true" class="invisible h-0 shrink-0 p-1.5">
          {#each sortedModels as model (model.value)}
            <li class="flex items-center gap-2 px-2.5 py-2 text-sm">
              <span class="size-4 shrink-0"></span>
              <span class="whitespace-nowrap">{model.label}</span>
              {#if model.value === chatboxStore.model}
                <span class="size-4 shrink-0"></span>
              {/if}
            </li>
          {/each}
        </ul>
        <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
        <ul
          class="flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto p-1.5"
          tabindex="0"
          role="listbox"
          aria-label="Available models"
          onkeydown={handleListKeydown}
        >
          {#if chatbox.isModelsLoading}
            <li class="px-3 py-2 text-sm text-muted-foreground">
              <Icon icon="fa6-solid:spinner" class="mr-1.5 inline animate-spin" />
              Loading models…
            </li>
          {:else if chatbox.models.length === 0}
            <li class="px-3 py-2 text-sm text-muted-foreground">
              No models available
            </li>
          {:else}
            {#each sortedModels as model (model.value)}
              {@const isSelected = model.value === chatboxStore.model}
              {@const isPreview = model.value === previewModel?.value}
              <li role="option" aria-selected={isSelected}>
                <button
                  type="button"
                  class={cn(
                    "flex w-full items-center gap-2 rounded-md px-2.5 py-2 text-left text-sm transition-colors",
                    "hover:bg-muted dark:hover:bg-muted/50",
                    isPreview && "bg-muted dark:bg-muted/50",
                  )}
                  onmouseenter={() => (previewValue = model.value)}
                  onfocus={() => (previewValue = model.value)}
                  onclick={() => selectModel(model.value)}
                >
                  <img
                    src={modelFamilyLogo(model.family)}
                    alt=""
                    class="size-4 shrink-0 object-contain"
                  />
                  <span class="min-w-0 grow truncate">{model.label}</span>
                  {#if isSelected}
                    <Icon
                      icon="fa6-solid:check"
                      class="size-4 shrink-0 text-primary"
                    />
                  {/if}
                </button>
              </li>
            {/each}
          {/if}
        </ul>
      </div>

      <!-- Model details -->
      <div class="flex flex-1 flex-col">
        {#if previewModel}
          <ScrollArea orientation="vertical" class="flex-1">
            <div class="flex flex-col gap-4 p-4">
              <div class="flex items-start gap-3">
                <img
                  src={modelFamilyLogo(previewModel.family)}
                  alt=""
                  class="size-9 shrink-0 object-contain"
                />
                <div class="flex flex-col gap-1">
                  <h3 class="text-base leading-tight font-semibold">
                    {previewModel.label}
                  </h3>
                  {#if previewModel.provider || previewModel.family}
                    <p class="text-xs text-muted-foreground">
                      {[previewModel.provider, previewModel.family]
                        .filter(Boolean)
                        .join(" · ")}
                    </p>
                  {/if}
                </div>
              </div>

              {#if previewModel.description}
                <p class="text-sm leading-relaxed text-muted-foreground">
                  {previewModel.description}
                </p>
              {/if}

              {#if previewModel.modalities.length > 0 || previewModel.features.length > 0}
                <div class="flex flex-col gap-2">
                  <span class="text-xs font-medium text-muted-foreground">
                    Tags
                  </span>
                  <div class="flex flex-wrap gap-1.5">
                    {#each previewModel.modalities as modality (modality)}
                      <Badge variant="secondary">{modality}</Badge>
                    {/each}
                    {#each previewModel.features as feature (feature)}
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
                {#if previewModel.reasoning}
                  <div class="flex items-center justify-between gap-3">
                    <div class="flex flex-col">
                      <span class="text-sm font-medium">Reasoning</span>
                      <span class="text-xs text-muted-foreground">
                        Let the model think through problems step by step.
                      </span>
                    </div>
                    <button
                      type="button"
                      role="switch"
                      aria-checked={reasoningEnabled}
                      aria-label="Toggle reasoning"
                      onclick={toggleReasoning}
                      class={cn(
                        "relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors",
                        reasoningEnabled
                          ? "bg-primary"
                          : "bg-muted-foreground/30",
                      )}
                    >
                      <span
                        class={cn(
                          "inline-block size-4 rounded-full bg-white shadow transition-transform",
                          reasoningEnabled
                            ? "translate-x-4"
                            : "translate-x-0.5",
                        )}
                      ></span>
                    </button>
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
                "flex w-full items-center justify-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                previewModel.value === chatboxStore.model
                  ? "bg-muted text-muted-foreground"
                  : "bg-primary text-primary-foreground hover:bg-primary/90",
              )}
              disabled={previewModel.value === chatboxStore.model}
              onclick={() => selectModel(previewModel.value)}
            >
              {#if previewModel.value === chatboxStore.model}
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
      </div>
    </PopoverContent>
  </PopoverPositioner>
</Popover>
