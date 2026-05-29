<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

// TooltipProvider is a no-op in Ark UI Svelte - Ark handles its own context
let { children }: { children?: Snippet } = $props();
</script>

{@render children?.()}
