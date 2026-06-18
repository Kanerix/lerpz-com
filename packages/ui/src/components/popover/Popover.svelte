<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";

let {
    children,
    ...rest
}: { children?: Snippet; [key: string]: unknown } = $props();
</script>

<Popover.Root {...rest}>
  {@render children?.()}
</Popover.Root>
