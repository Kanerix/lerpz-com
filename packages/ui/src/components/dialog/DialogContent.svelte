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

<Dialog.Content
  data-slot="dialog-content"
  class={cn(
    "relative flex w-full max-w-lg flex-col",
    "bg-popover text-popover-foreground",
    "rounded-lg border shadow-2xl outline-none",
    "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
    "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Dialog.Content>
