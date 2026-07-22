<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import { Portal } from "@ark-ui/svelte/portal";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Portal>
  <Menu.Positioner class="isolate z-50">
    <Menu.Content
      data-slot="dropdown-menu-content"
      class={cn(
        "z-50 min-w-48 p-1 bg-popover text-popover-foreground",
        "rounded-lg border shadow-2xl outline-none",
        "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
        "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
        className
      )}
      {...rest}
    >
      {@render children?.()}
    </Menu.Content>
  </Menu.Positioner>
</Portal>
