<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>
<div data-slot="card-title" class={cn("text-base font-semibold leading-none", className)} {...rest}>{@render children?.()}</div>
