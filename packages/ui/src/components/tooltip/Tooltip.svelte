<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { TooltipRootProps } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";

// openDelay/closeDelay are valid zag-js props that the Svelte wrapper types don't surface directly
const rootProps = { openDelay: 0, closeDelay: 0 } as TooltipRootProps;

let { children }: { children?: Snippet } = $props();
</script>

<Tooltip.Root {...rootProps}>
  {@render children?.()}
</Tooltip.Root>
