<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    value,
    disabled = false,
    onclick,
    children,
    ...rest
}: {
    class?: string;
    value?: string;
    disabled?: boolean;
    onclick?: () => void;
    children?: Snippet;
    [key: string]: unknown;
} = $props();
</script>

<Menu.Item
  data-slot="dropdown-menu-item"
  {value}
  {disabled}
  {onclick}
  class={cn(
    "focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground relative flex cursor-pointer select-none items-center gap-2 rounded-xl px-3 py-2 text-sm outline-none transition-colors",
    "[&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0",
    "data-disabled:pointer-events-none data-disabled:opacity-50",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Menu.Item>
