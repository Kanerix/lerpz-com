<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    isActive = false,
    children,
    ...rest
}: {
    class?: string;
    isActive?: boolean;
    children?: Snippet;
    [key: string]: unknown;
} = $props();

const classes = $derived(
    cn(
        "flex h-7 min-w-0 items-center gap-2 overflow-hidden rounded-md px-2 text-sm",
        "text-sidebar-foreground outline-none ring-sidebar-ring",
        "transition-colors focus-visible:ring-2 cursor-pointer",
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "active:bg-sidebar-accent active:text-sidebar-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-50",
        "aria-disabled:pointer-events-none aria-disabled:opacity-50",
        "data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium",
        "data-[active=true]:text-sidebar-accent-foreground",
        "[&>svg]:size-4 [&>svg]:shrink-0 [&>svg]:text-sidebar-foreground/70",
        "group-data-[state=collapsed]:hidden",
        className,
    ),
);
</script>

{#if rest.href}
<a
  data-slot="sidebar-menu-sub-button"
  data-sidebar="menu-sub-button"
  data-active={isActive}
  class={classes}
  {...rest}
>
  {@render children?.()}
</a>
{:else}
<button
  data-slot="sidebar-menu-sub-button"
  data-sidebar="menu-sub-button"
  data-active={isActive}
  type="button"
  class={classes}
  {...rest}
>
  {@render children?.()}
</button>
{/if}
