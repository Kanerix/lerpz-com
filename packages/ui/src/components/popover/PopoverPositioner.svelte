<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";

let { children, ...rest }: { children?: Snippet; [key: string]: unknown } =
    $props();
</script>

<Popover.Positioner data-slot="popover-positioner" class="isolate z-50" {...rest}>
  {@render children?.()}
</Popover.Positioner>
