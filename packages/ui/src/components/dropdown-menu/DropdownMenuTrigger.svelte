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

<Menu.Trigger data-slot="dropdown-menu-trigger" class={cn(className)} {...rest}>
  {@render children?.()}
</Menu.Trigger>
