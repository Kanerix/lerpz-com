<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<ul
  data-slot="sidebar-menu-sub"
  class={cn(
    "mx-3.5 flex min-w-0 flex-col gap-1 border-l border-sidebar-border px-2.5 py-0.5",
    "group-data-[state=collapsed]:hidden",
    className
  )}
  {...rest}
>
  {@render children?.()}
</ul>
