<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { cn } from "@lerpz/ui/lib/utils";
import ModelSelector from "$lib/components/model-selector/ModelSelector.svelte";
import EaselAspectRatio from "./EaselAspectRatio.svelte";
import {
    easelStore,
    MAX_IMAGE_COUNT,
    MIN_IMAGE_COUNT,
} from "./easel.store.svelte.js";
import { getEaselContext } from "./easel-context.svelte.js";

let {
    enhance,
    isPending,
    isEnhancePending,
}: {
    enhance?: (prompt: string) => Promise<string>;
    isPending: boolean;
    isEnhancePending: boolean;
} = $props();

const easel = getEaselContext();

async function handleEnhance() {
    if (!easelStore.prompt.trim() || !enhance) return;
    const newPrompt = await enhance(easelStore.prompt);
    easelStore.setPrompt(newPrompt);
}
</script>

<div class="flex flex-wrap items-center gap-2">
  <ModelSelector
    models={easel.models}
    isModelsLoading={easel.isModelsLoading}
    value={easelStore.model}
    onSelect={(value) => easelStore.setModel(value)}
  />
  <EaselAspectRatio disabled={isPending} />

  <!-- Image count stepper -->
  <div
    class="inline-flex h-9 items-center gap-1 rounded-4xl border border-sidebar-border px-1"
    role="group"
    aria-label="Number of images"
  >
    <button
      type="button"
      class={cn(
        "flex size-7 items-center justify-center rounded-full transition-colors",
        "text-muted-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-40",
      )}
      aria-label="Fewer images"
      disabled={isPending || easelStore.count <= MIN_IMAGE_COUNT}
      onclick={() => easelStore.setCount(easelStore.count - 1)}
    >
      <Icon icon="fa6-solid:minus" class="size-3" />
    </button>
    <span class="flex items-center gap-1 px-1 text-sm tabular-nums">
      <Icon icon="fa6-regular:images" class="size-3.5 text-muted-foreground" />
      {easelStore.count}
    </span>
    <button
      type="button"
      class={cn(
        "flex size-7 items-center justify-center rounded-full transition-colors",
        "text-muted-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-40",
      )}
      aria-label="More images"
      disabled={isPending || easelStore.count >= MAX_IMAGE_COUNT}
      onclick={() => easelStore.setCount(easelStore.count + 1)}
    >
      <Icon icon="fa6-solid:plus" class="size-3" />
    </button>
  </div>

  <!-- Enhance -->
  <div class="ml-auto">
    <Tooltip>
      <TooltipTrigger>
        <Button
          variant="ghost"
          size="icon"
          aria-label="Enhance prompt"
          disabled={isPending || !easelStore.prompt.trim() || !enhance}
          onclick={handleEnhance}
        >
          {#if isEnhancePending}
            <Icon icon="fa6-solid:spinner" class="animate-spin" />
          {:else}
            <Icon icon="fa6-solid:wand-magic-sparkles" />
          {/if}
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Enhance the prompt!</p></TooltipContent>
    </Tooltip>
  </div>
</div>
