<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    side = "top",
    sideOffset = 4,
    children,
}: {
    class?: string;
    side?: "top" | "bottom" | "left" | "right";
    sideOffset?: number;
    children?: Snippet;
} = $props();
</script>

<Tooltip.Positioner {side} {sideOffset} class="isolate z-50">
  <Tooltip.Content
    data-slot="tooltip-content"
    class={cn(
      "bg-foreground text-background z-50 w-fit max-w-xs rounded-2xl px-3 py-1.5 text-xs",
      "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
      "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      className
    )}
  >
    {@render children?.()}
  </Tooltip.Content>
</Tooltip.Positioner>
