<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import type { Snippet } from "svelte";

let {
    side = "bottom",
    align = "start",
    sideOffset = 4,
    children,
    ...rest
}: {
    side?: "top" | "bottom" | "left" | "right";
    align?: "start" | "center" | "end";
    sideOffset?: number;
    children?: Snippet;
    [key: string]: unknown;
} = $props();

// Ark UI configures placement on the Root via the `positioning` prop; the
// Positioner itself is a bare wrapper.
const positioning = $derived({
    placement: align === "center" ? side : (`${side}-${align}` as const),
    gutter: sideOffset,
});
</script>

<Menu.Root {positioning} {...rest}>
  {@render children?.()}
</Menu.Root>
