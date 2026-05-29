<script lang="ts">
import type { PropsFn } from "@ark-ui/svelte";
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    asChild,
    ...rest
}: {
    class?: string;
    children?: Snippet;
    asChild?: Snippet<[PropsFn<"button">]>;
    [key: string]: unknown;
} = $props();
</script>

<Tooltip.Trigger data-slot="tooltip-trigger" class={cn(className)} {asChild} {...rest}>
  {@render children?.()}
</Tooltip.Trigger>
