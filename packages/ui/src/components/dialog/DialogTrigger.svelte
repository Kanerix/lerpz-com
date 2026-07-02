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

<Dialog.Trigger
  data-slot="dialog-trigger"
  class={cn("cursor-pointer", className)}
  {...rest}
>
  {@render children?.()}
</Dialog.Trigger>
