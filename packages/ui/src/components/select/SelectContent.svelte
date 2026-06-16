<script lang="ts">
import { Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    align = "center",
    sideOffset = 4,
    children,
}: {
    class?: string;
    align?: "start" | "center" | "end";
    sideOffset?: number;
    children?: Snippet;
} = $props();
</script>

<Select.Positioner {align} {sideOffset} class="isolate z-50">
  <Select.Content
    data-slot="select-content"
    class={cn(
      "z-50 min-w-36 bg-popover text-popover-foreground",
      "rounded-2xl border shadow-2xl outline-none",
      "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
      "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      className
    )}
  >
    <Select.List>
      {@render children?.()}
    </Select.List>
  </Select.Content>
</Select.Positioner>
