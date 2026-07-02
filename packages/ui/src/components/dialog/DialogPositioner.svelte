<script lang="ts">
import { Dialog } from "@ark-ui/svelte/dialog";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Dialog.Positioner
  data-slot="dialog-positioner"
  class={cn("fixed inset-0 z-50 flex items-center justify-center p-4", className)}
  {...rest}
>
  {@render children?.()}
</Dialog.Positioner>
