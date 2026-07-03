<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let { children }: { children?: Snippet } = $props();
</script>

<main
  data-slot="sidebar-inset"
  class={cn(
    "relative flex flex-1 flex-col overflow-auto min-h-screen min-w-0",
    "bg-background"
  )}
>
  {@render children?.()}
</main>
