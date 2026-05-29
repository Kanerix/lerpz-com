<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>
<ul data-slot="sidebar-menu" class={cn("flex w-full min-w-0 flex-col gap-1", className)} {...rest}>{@render children?.()}</ul>
