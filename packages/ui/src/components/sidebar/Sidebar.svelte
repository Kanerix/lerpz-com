<script lang="ts">
import type { Snippet } from "svelte";
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
</script>

<div
  data-slot="sidebar"
  data-state={sidebar.state}
  data-collapsible={collapsible}
  class="
    group peer text-sidebar-foreground hidden md:flex
    w-[--sidebar-width] flex-col
    data-[collapsible=icon]:data-[state=collapsed]:w-[--sidebar-width-icon]
    transition-[width] duration-200
    {className}
  "
  {...rest}
>
  <div class="flex h-full w-full flex-col bg-sidebar border-r border-sidebar-border shadow-[1px_0_8px_0_rgba(0,0,0,0.06)]">
    {@render children?.()}
  </div>
</div>
