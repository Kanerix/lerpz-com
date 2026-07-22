<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Tooltip.Positioner class="isolate z-50">
  <Tooltip.Content
    data-slot="tooltip-content"
    class={cn(
      "z-50 w-fit max-w-xs px-3 py-1.5 text-xs",
      "bg-foreground text-background rounded-lg",
      "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
      "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      className
    )}
    {...rest}
  >
    {@render children?.()}
  </Tooltip.Content>
</Tooltip.Positioner>
