<script lang="ts">
import { Popover } from "@ark-ui/svelte/popover";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Popover.Content
  data-slot="popover-content"
  class={cn(
    "bg-popover text-popover-foreground z-50 rounded-2xl border shadow-2xl outline-none",
    "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
    "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Popover.Content>
