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
    clapperStore,
    DURATIONS,
    resolveDuration,
} from "./clapper.store.svelte.js";

let { disabled = false }: { disabled?: boolean } = $props();

const selected = $derived(resolveDuration(clapperStore.duration));
</script>

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
    aria-label="Clip duration"
  >
    <Icon icon="fa6-regular:clock" class="size-3.5 text-muted-foreground" />
    <span>{selected.value}s</span>
    <Icon icon="fa6-solid:angle-up" class="size-4 opacity-60" />
  </DropdownMenuTrigger>

  <DropdownMenuContent class="w-44">
    {#each DURATIONS as duration (duration.value)}
      {@const isSelected = duration.value === clapperStore.duration}
      <DropdownMenuItem
        value={String(duration.value)}
        onclick={() => clapperStore.setDuration(duration.value)}
      >
        <Icon
          icon="fa6-regular:clock"
          class="size-3.5 text-muted-foreground"
        />
        <span class="grow font-medium">{duration.label}</span>
        {#if isSelected}
          <Icon icon="fa6-solid:check" class="size-4 shrink-0 text-primary" />
        {/if}
      </DropdownMenuItem>
    {/each}
  </DropdownMenuContent>
</DropdownMenu>
