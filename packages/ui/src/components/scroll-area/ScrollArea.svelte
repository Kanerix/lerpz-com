<script lang="ts">
import { ScrollArea } from "@ark-ui/svelte/scroll-area";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    orientation = "both",
    children,
}: {
    class?: string;
    orientation?: "vertical" | "horizontal" | "both";
    children?: Snippet;
} = $props();
</script>

<ScrollArea.Root
  data-slot="scroll-area"
  class={cn("relative overflow-hidden", className)}
>
  <ScrollArea.Viewport
    class="size-full rounded-[inherit] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
  >
    <ScrollArea.Content
      class={cn(orientation === "vertical" && "w-full min-w-0!")}
    >
      {@render children?.()}
    </ScrollArea.Content>
  </ScrollArea.Viewport>
  {#if orientation === "vertical" || orientation === "both"}
    <ScrollArea.Scrollbar
      orientation="vertical"
      class="hidden w-2.5 p-0.5 select-none touch-none transition-colors data-[overflow-y]:flex"
    >
      <ScrollArea.Thumb class="relative flex-1 bg-border rounded-full" />
    </ScrollArea.Scrollbar>
  {/if}
  {#if orientation === "horizontal" || orientation === "both"}
    <ScrollArea.Scrollbar
      orientation="horizontal"
      class="hidden flex-col h-2.5 p-0.5 select-none touch-none transition-colors data-[overflow-x]:flex"
    >
      <ScrollArea.Thumb class="relative flex-1 bg-border rounded-full" />
    </ScrollArea.Scrollbar>
  {/if}
  <ScrollArea.Corner />
</ScrollArea.Root>
