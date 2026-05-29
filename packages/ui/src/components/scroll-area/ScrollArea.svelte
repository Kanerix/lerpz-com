<script lang="ts">
import { ScrollArea } from "@ark-ui/svelte/scroll-area";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
}: { class?: string; children?: Snippet } = $props();
</script>

<ScrollArea.Root data-slot="scroll-area" class={cn("relative overflow-hidden", className)}>
  <ScrollArea.Viewport class="size-full rounded-[inherit]">
    {@render children?.()}
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar
    orientation="vertical"
    class="flex touch-none select-none p-0.5 transition-colors w-2.5"
  >
    <ScrollArea.Thumb class="bg-border relative flex-1 rounded-full" />
  </ScrollArea.Scrollbar>
  <ScrollArea.Scrollbar
    orientation="horizontal"
    class="flex touch-none select-none p-0.5 transition-colors h-2.5 flex-col"
  >
    <ScrollArea.Thumb class="bg-border relative flex-1 rounded-full" />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>
