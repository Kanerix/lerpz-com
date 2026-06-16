<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<ul
    data-slot="sidebar-menu"
    class={cn("flex flex-col gap-1 w-full min-w-0", className)}
    {...rest}
>
    {@render children?.()}
</ul>
