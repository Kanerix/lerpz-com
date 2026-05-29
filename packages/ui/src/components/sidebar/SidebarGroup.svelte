<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>
<div data-slot="sidebar-group" class={cn("relative flex w-full min-w-0 flex-col p-2", className)} {...rest}>{@render children?.()}</div>
