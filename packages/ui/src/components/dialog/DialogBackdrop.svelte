<script lang="ts">
import { Dialog } from "@ark-ui/svelte/dialog";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    ...rest
}: { class?: string; [key: string]: unknown } = $props();
</script>

<Dialog.Backdrop
  data-slot="dialog-backdrop"
  class={cn(
    "fixed inset-0 z-50 bg-black/50 backdrop-blur-sm",
    "data-[state=open]:animate-in data-[state=open]:fade-in-0",
    "data-[state=closed]:animate-out data-[state=closed]:fade-out-0",
    className
  )}
  {...rest}
/>
