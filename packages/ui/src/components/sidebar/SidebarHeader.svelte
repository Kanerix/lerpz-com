<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>
<div data-slot="sidebar-header" class={cn("flex flex-col gap-2 p-2", className)} {...rest}>{@render children?.()}</div>
