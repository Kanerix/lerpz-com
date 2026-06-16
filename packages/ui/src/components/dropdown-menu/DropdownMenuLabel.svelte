<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Menu.ItemGroupLabel
  data-slot="dropdown-menu-label"
  class={cn("px-3 py-2 text-xs text-muted-foreground", className)}
  {...rest}
>
  {@render children?.()}
</Menu.ItemGroupLabel>
