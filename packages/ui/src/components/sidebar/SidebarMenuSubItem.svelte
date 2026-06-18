<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<li
  data-slot="sidebar-menu-sub-item"
  class={cn("group/menu-sub-item relative", className)}
  {...rest}
>
  {@render children?.()}
</li>
