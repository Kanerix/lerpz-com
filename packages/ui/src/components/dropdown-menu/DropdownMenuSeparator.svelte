<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    ...rest
}: { class?: string; [key: string]: unknown } = $props();
</script>

<Menu.Separator
  data-slot="dropdown-menu-separator"
  class={cn("-mx-1 my-1 h-px bg-border/50", className)}
  {...rest}
/>
