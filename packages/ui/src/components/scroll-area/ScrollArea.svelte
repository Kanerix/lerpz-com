<script lang="ts">
import { ScrollArea } from "@ark-ui/svelte/scroll-area";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
}: { class?: string; children?: Snippet } = $props();
</script>

<ScrollArea.Root
  data-slot="scroll-area"
  class={cn("relative overflow-hidden", className)}
>
  <ScrollArea.Viewport class="size-full rounded-[inherit]">
    {@render children?.()}
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar
    orientation="vertical"
    class="flex w-2.5 p-0.5 select-none touch-none transition-colors"
  >
    <ScrollArea.Thumb class="relative flex-1 bg-border rounded-full" />
  </ScrollArea.Scrollbar>
  <ScrollArea.Scrollbar
    orientation="horizontal"
    class="flex flex-col h-2.5 p-0.5 select-none touch-none transition-colors"
  >
    <ScrollArea.Thumb class="relative flex-1 bg-border rounded-full" />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>
