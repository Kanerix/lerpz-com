<script lang="ts">
import { cva } from "class-variance-authority";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";

const variants = cva(
    [
        "peer/menu-button flex w-full items-center gap-2 overflow-hidden",
        "group-data-[state=collapsed]:justify-center",
        "rounded-lg p-2 text-left text-sm outline-none ring-sidebar-ring",
        "cursor-pointer",
        "transition-[width,height,padding] focus-visible:ring-2",
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "active:bg-sidebar-accent active:text-sidebar-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-50",
        "aria-disabled:pointer-events-none aria-disabled:opacity-50",
        "group-has-[[data-sidebar=menu-action]]/menu-item:pr-8",
        "data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium",
        "data-[active=true]:text-sidebar-accent-foreground",
    ],
    {
        variants: {
            size: {
                default: "h-8 text-sm",
                sm: "h-7 text-xs",
                lg: "h-12 text-sm",
            },
        },
        defaultVariants: { size: "default" },
    },
);

let {
    class: className = "",
    size = "default",
    isActive = false,
    children,
    ...rest
}: {
    class?: string;
    size?: "default" | "sm" | "lg";
    isActive?: boolean;
    children?: Snippet;
    [key: string]: unknown;
} = $props();

const sidebar = useSidebar();
</script>

{#if rest.href}
<a
  data-slot="sidebar-menu-button"
  data-sidebar="menu-button"
  data-active={isActive}
  class={cn(variants({ size }), className)}
  {...rest}
>
  {@render children?.()}
</a>
{:else}
<button
  data-slot="sidebar-menu-button"
  data-sidebar="menu-button"
  data-active={isActive}
  type="button"
  class={cn(variants({ size }), className)}
  {...rest}
>
  {@render children?.()}
</button>
{/if}
