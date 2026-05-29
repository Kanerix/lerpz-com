<script lang="ts">
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    ...rest
}: { class?: string; [key: string]: unknown } = $props();
</script>

<div data-slot="popover-backdrop" class={cn("fixed inset-0 z-50", className)} {...rest}></div>
