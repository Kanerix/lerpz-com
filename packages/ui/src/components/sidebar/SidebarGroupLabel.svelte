<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
const sidebar = useSidebar();
</script>
<div
  data-slot="sidebar-group-label"
  data-sidebar="group-label"
  class={cn(
    "text-sidebar-foreground/70 ring-sidebar-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-none",
    sidebar.state === "collapsed" && "opacity-0 overflow-hidden h-0",
    className
  )}
  {...rest}
>
  {@render children?.()}
</div>
