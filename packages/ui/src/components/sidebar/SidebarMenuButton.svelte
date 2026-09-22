<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";
import {
    type SidebarMenuButtonVariants,
    sidebarMenuButtonVariants,
} from "./sidebar-menu-button-variants.js";

let {
    class: className = "",
    size = "default",
    isActive = false,
    children,
    ...rest
}: {
    class?: string;
    isActive?: boolean;
    children?: Snippet;
    [key: string]: unknown;
} & SidebarMenuButtonVariants = $props();

const sidebar = useSidebar();
</script>

{#if rest.href}
<a
  data-slot="sidebar-menu-button"
  data-sidebar="menu-button"
  data-active={isActive}
  class={cn(sidebarMenuButtonVariants({ size }), className)}
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
  class={cn(sidebarMenuButtonVariants({ size }), className)}
  {...rest}
>
  {@render children?.()}
</button>
{/if}
