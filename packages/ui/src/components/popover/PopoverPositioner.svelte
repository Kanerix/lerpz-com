<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";

let {
    side = "bottom",
    sideOffset = 8,
    align = "center",
    anchor,
    children,
}: {
    side?: "top" | "bottom" | "left" | "right";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    anchor?: HTMLElement | null;
    children?: Snippet;
} = $props();
</script>

<Popover.Positioner
  data-slot="popover-positioner"
  {side}
  {sideOffset}
  {align}
  anchorElement={anchor}
  class="isolate z-50"
>
  {@render children?.()}
</Popover.Positioner>
