<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";

let {
    collapsible = "offcanvas",
    class: className = "",
    children,
    ...rest
}: {
    collapsible?: "offcanvas" | "icon" | "none";
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
} = $props();

const sidebar = useSidebar();

const width = $derived(
    collapsible === "icon" && sidebar.state === "collapsed"
        ? "var(--sidebar-width-icon)"
        : "var(--sidebar-width)",
);
</script>

<div
  data-slot="sidebar"
  data-state={sidebar.state}
  data-collapsible={collapsible}
  style="width: {width}; min-width: 0;"
  class={cn(
    "group peer hidden md:flex flex-col shrink-0 grow-0",
    "sticky top-0 h-svh",
    "text-sidebar-foreground transition-[width] duration-200",
    className
  )}
  {...rest}
>
  <div
    class={cn(
      "flex h-full w-full flex-col bg-sidebar",
      "border-r border-sidebar-border",
      "shadow-[1px_0_8px_0_rgba(0,0,0,0.06)]"
    )}
  >
    {@render children?.()}
  </div>
</div>
