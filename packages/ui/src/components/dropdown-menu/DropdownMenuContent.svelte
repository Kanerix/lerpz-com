<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    align = "start",
    side = "bottom",
    sideOffset = 4,
    children,
}: {
    class?: string;
    align?: "start" | "center" | "end";
    side?: "top" | "bottom" | "left" | "right";
    sideOffset?: number;
    children?: Snippet;
} = $props();
</script>

<Menu.Positioner {align} {side} {sideOffset} class="isolate z-50">
  <Menu.Content
    data-slot="dropdown-menu-content"
    class={cn(
      "z-50 min-w-48 p-1 bg-popover text-popover-foreground",
      "rounded-2xl border shadow-2xl outline-none",
      "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
      "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      className
    )}
  >
    {@render children?.()}
  </Menu.Content>
</Menu.Positioner>
