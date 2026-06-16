<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<div
    data-slot="select-group"
    class={cn("p-1", className)}
    {...rest}
>
    {@render children?.()}
</div>
