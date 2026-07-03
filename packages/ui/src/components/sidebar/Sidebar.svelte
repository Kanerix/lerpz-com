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

const isIcon = $derived(collapsible === "icon");

// Width reserved in the layout flow. Uses the persistent open state so the
// page content doesn't shift when the sidebar merely expands on hover.
const reservedWidth = $derived(
    isIcon && sidebar.openState === "collapsed"
        ? "var(--sidebar-width-icon)"
        : "var(--sidebar-width)",
);

// Visual width of the panel. Follows the effective state, which includes
// hover-to-expand on desktop.
const visualWidth = $derived(
    isIcon && sidebar.state === "collapsed"
        ? "var(--sidebar-width-icon)"
        : "var(--sidebar-width)",
);

// True while the panel is expanded purely because of hover (persistent state
// is still collapsed). Used to lift the panel above the content as an overlay.
const isHoverExpanded = $derived(
    isIcon && sidebar.openState === "collapsed" && sidebar.state === "expanded",
);

const canHoverExpand = $derived(isIcon && !sidebar.isMobile);

function handleEnter() {
    if (canHoverExpand) sidebar.setHovered(true);
}

function handleLeave() {
    if (canHoverExpand) sidebar.setHovered(false);
}
</script>

<!-- Space reserver: keeps content beside the sidebar without shifting on hover. -->
<div
  data-slot="sidebar"
  data-state={sidebar.state}
  data-collapsible={collapsible}
  style="width: {reservedWidth}; min-width: 0;"
  class={cn(
    "group peer relative hidden md:block shrink-0 grow-0",
    "sticky top-0 h-svh",
    "text-sidebar-foreground transition-[width] duration-200",
    // The sticky wrapper forms its own stacking context; keep it above the
    // inset so the hover-expanded panel can overlay the page content.
    isHoverExpanded ? "z-40" : "z-20",
    className
  )}
  onmouseenter={handleEnter}
  onmouseleave={handleLeave}
  {...rest}
>
  <!-- Visual panel: overlays the content when expanded on hover. -->
  <div
    style="width: {visualWidth};"
    class={cn(
      "absolute inset-y-0 left-0 flex h-full flex-col bg-sidebar",
      "border-r border-sidebar-border",
      "transition-[width,box-shadow] duration-200",
      isHoverExpanded
        ? "shadow-[4px_0_24px_-2px_rgba(0,0,0,0.18)]"
        : "shadow-[1px_0_8px_0_rgba(0,0,0,0.06)]"
    )}
  >
    {@render children?.()}
  </div>
</div>
