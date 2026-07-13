<script lang="ts">
import Icon from "@iconify/svelte";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import { cn } from "@lerpz/ui/lib/utils";
import {
    ASPECT_RATIOS,
    type AspectRatioOption,
    easelStore,
    resolveAspectRatio,
} from "./easel.store.svelte.js";

let { disabled = false }: { disabled?: boolean } = $props();

const selected = $derived<AspectRatioOption>(
    resolveAspectRatio(easelStore.aspectRatio),
);
</script>

{#snippet glyph(ratio: AspectRatioOption, size: number)}
  {@const scale = size / Math.max(ratio.w, ratio.h)}
  <span
    class="inline-block shrink-0 rounded-[3px] border-[1.5px] border-current"
    style="width: {ratio.w * scale}px; height: {ratio.h * scale}px;"
  ></span>
{/snippet}

<DropdownMenu>
  <DropdownMenuTrigger
    {disabled}
    class={cn(
      "inline-flex h-9 items-center gap-1.5 rounded-4xl px-3 text-sm",
      "border border-transparent bg-transparent transition-colors",
      "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
      "data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground",
      "disabled:pointer-events-none disabled:opacity-50",
    )}
    aria-label="Aspect ratio"
  >
    {@render glyph(selected, 16)}
    <span>{selected.value}</span>
    <Icon icon="fa6-solid:angle-up" class="size-4 opacity-60" />
  </DropdownMenuTrigger>

  <DropdownMenuContent class="w-48">
    {#each ASPECT_RATIOS as ratio (ratio.value)}
      {@const isSelected = ratio.value === easelStore.aspectRatio}
      <DropdownMenuItem
        value={ratio.value}
        onclick={() => easelStore.setAspectRatio(ratio.value)}
      >
        <span class="flex size-4 items-center justify-center text-muted-foreground">
          {@render glyph(ratio, 16)}
        </span>
        <span class="grow font-medium">{ratio.value}</span>
        <span class="text-xs text-muted-foreground">{ratio.label}</span>
        {#if isSelected}
          <Icon icon="fa6-solid:check" class="size-4 shrink-0 text-primary" />
        {/if}
      </DropdownMenuItem>
    {/each}
  </DropdownMenuContent>
</DropdownMenu>
