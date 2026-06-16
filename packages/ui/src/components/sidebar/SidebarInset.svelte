<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let { children }: { children?: Snippet } = $props();
</script>

<main
  data-slot="sidebar-inset"
  class={cn(
    "relative flex flex-1 flex-col overflow-auto min-h-screen",
    "bg-background transition-[margin] duration-200",
    "peer-data-[state=collapsed]:md:ml-[--sidebar-width-icon]",
    "peer-data-[state=expanded]:md:ml-[--sidebar-width]"
  )}
>
  {@render children?.()}
</main>
