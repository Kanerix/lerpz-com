<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import { Portal } from "@ark-ui/svelte/portal";
import type { Snippet } from "svelte";

let { children, ...rest }: { children?: Snippet; [key: string]: unknown } =
    $props();
</script>

<Portal>
  <Popover.Positioner data-slot="popover-positioner" class="isolate z-50" {...rest}>
    {@render children?.()}
  </Popover.Positioner>
</Portal>
