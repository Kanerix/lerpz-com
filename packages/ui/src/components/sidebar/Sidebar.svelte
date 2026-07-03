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

// Close the mobile drawer when a navigation link inside it is activated.
function handleDrawerClick(event: MouseEvent) {
    if ((event.target as HTMLElement | null)?.closest("a")) {
        sidebar.setOpenMobile(false);
    }
}
</script>

{#if sidebar.isMobile}
  <button
    type="button"
    aria-label="Close sidebar"
    tabindex={sidebar.openMobile ? 0 : -1}
    onclick={() => sidebar.setOpenMobile(false)}
    class={cn(
      "fixed inset-0 z-40 bg-black/50 md:hidden",
      "transition-opacity duration-200",
      sidebar.openMobile ? "opacity-100" : "pointer-events-none opacity-0"
    )}
  ></button>
  <div
    data-slot="sidebar"
    data-state="expanded"
    data-collapsible={collapsible}
    style="width: var(--sidebar-width);"
    onclick={handleDrawerClick}
    class={cn(
      "group fixed inset-y-0 left-0 z-50 flex h-svh flex-col md:hidden",
      "bg-sidebar text-sidebar-foreground",
      "border-r border-sidebar-border shadow-xl",
      "transition-transform duration-200",
      sidebar.openMobile ? "translate-x-0" : "-translate-x-full",
      className
    )}
    {...rest}
  >
    {@render children?.()}
  </div>
{:else}
  <div
    data-slot="sidebar"
    data-state={sidebar.state}
    data-collapsible={collapsible}
    style="width: {reservedWidth}; min-width: 0;"
    class={cn(
      "group peer relative hidden md:block shrink-0 grow-0",
      "sticky top-0 h-svh",
      "text-sidebar-foreground transition-[width] duration-200",
      isHoverExpanded ? "z-40" : "z-20",
      className
    )}
    onmouseenter={handleEnter}
    onmouseleave={handleLeave}
    {...rest}
  >
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
{/if}
