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
    data-slot="sidebar-content"
    class={cn("flex flex-1 flex-col gap-2 overflow-auto min-h-0", className)}
    {...rest}
>
    {@render children?.()}
</div>
