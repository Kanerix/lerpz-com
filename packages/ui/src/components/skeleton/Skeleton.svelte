<script lang="ts">
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    ...rest
}: { class?: string; [key: string]: unknown } = $props();
</script>

<div
    data-slot="skeleton"
    class={cn("bg-accent rounded-lg animate-pulse", className)}
    {...rest}
></div>
