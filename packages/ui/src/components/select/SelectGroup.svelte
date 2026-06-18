<script lang="ts">
import { Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Select.ItemGroup data-slot="select-group" class={cn("p-1", className)} {...rest}>
  {@render children?.()}
</Select.ItemGroup>
