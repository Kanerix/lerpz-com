<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";

let {
    side = "top",
    align = "center",
    sideOffset = 4,
    openDelay = 0,
    closeDelay = 0,
    children,
}: {
    side?: "top" | "bottom" | "left" | "right";
    align?: "start" | "center" | "end";
    sideOffset?: number;
    openDelay?: number;
    closeDelay?: number;
    children?: Snippet;
} = $props();

// Ark UI configures placement on the Root via the `positioning` prop; the
// Positioner itself is a bare wrapper.
const positioning = $derived({
    placement: align === "center" ? side : (`${side}-${align}` as const),
    gutter: sideOffset,
});
</script>

<Tooltip.Root {openDelay} {closeDelay} {positioning}>
  {@render children?.()}
</Tooltip.Root>
