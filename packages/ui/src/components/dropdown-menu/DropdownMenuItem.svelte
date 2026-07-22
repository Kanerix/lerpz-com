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

// Ark UI's Menu.Item requires a unique `value`; fall back to a generated id
// when the consumer does not supply one.
const uid = $props.id();
</script>

<Menu.Item
  data-slot="dropdown-menu-item"
  value={value ?? uid}
  {disabled}
  {onclick}
  class={cn(
    "relative flex items-center gap-2 px-3 py-2 text-sm",
    "cursor-pointer select-none rounded-lg outline-none transition-colors",
    "[&_svg]:pointer-events-none [&_svg]:shrink-0",
    "[&_svg:not([class*='size-'])]:size-4",
    "focus:bg-accent focus:text-accent-foreground",
    "hover:bg-accent hover:text-accent-foreground",
    "data-disabled:pointer-events-none data-disabled:opacity-50",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Menu.Item>
