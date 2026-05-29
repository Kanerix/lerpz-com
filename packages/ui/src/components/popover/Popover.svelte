<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";

let { children }: { children?: Snippet } = $props();
</script>

<Popover.Root>
  {@render children?.()}
</Popover.Root>
