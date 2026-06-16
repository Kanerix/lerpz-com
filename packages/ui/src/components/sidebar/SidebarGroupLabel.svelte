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
    "flex h-8 shrink-0 items-center px-2 text-xs font-medium",
    "text-sidebar-foreground/70 rounded-md outline-none ring-sidebar-ring",
    sidebar.state === "collapsed" && "h-0 overflow-hidden opacity-0",
    className
  )}
  {...rest}
>
  {@render children?.()}
</div>
