<script lang="ts">
import { createListCollection, Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";

type Item = { value: string; label: string };

let {
    items = [],
    value = null,
    onValueChange,
    side = "bottom",
    align = "center",
    sideOffset = 4,
    children,
}: {
    items?: Item[];
    value?: string | null;
    onValueChange?: (value: string | null) => void;
    side?: "top" | "bottom" | "left" | "right";
    align?: "start" | "center" | "end";
    sideOffset?: number;
    children?: Snippet;
} = $props();

const collection = $derived(createListCollection({ items }));
const selectValue = $derived(value ? [value] : []);

// Ark UI configures placement on the Root via the `positioning` prop; the
// Positioner itself is a bare wrapper.
const positioning = $derived({
    placement: align === "center" ? side : (`${side}-${align}` as const),
    gutter: sideOffset,
});
</script>

<Select.Root
  {collection}
  value={selectValue}
  {positioning}
  onValueChange={(details: { value: string[] }) => onValueChange?.(details.value[0] ?? null)}
>
  {@render children?.()}
</Select.Root>
