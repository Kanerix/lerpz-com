<script lang="ts">
import { createListCollection, Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";

type Item = { value: string; label: string };

let {
    items = [],
    value = null,
    onValueChange,
    children,
}: {
    items?: Item[];
    value?: string | null;
    onValueChange?: (value: string | null) => void;
    children?: Snippet;
} = $props();

const collection = $derived(createListCollection({ items }));
const selectValue = $derived(value ? [value] : []);
</script>

<Select.Root
  {collection}
  value={selectValue}
  onValueChange={(details: { value: string[] }) => onValueChange?.(details.value[0] ?? null)}
>
  {@render children?.()}
</Select.Root>
