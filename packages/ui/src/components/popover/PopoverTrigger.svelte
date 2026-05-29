<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Popover.Trigger data-slot="popover-trigger" class={cn(className)} {...rest}>
  {@render children?.()}
</Popover.Trigger>
