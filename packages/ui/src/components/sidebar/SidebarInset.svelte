<script lang="ts">
import type { Snippet } from "svelte";
import { useSidebar } from "./context.svelte.js";

let { children }: { children?: Snippet } = $props();
const sidebar = useSidebar();
</script>

<main
  data-slot="sidebar-inset"
  class="bg-background relative flex min-h-screen flex-1 flex-col overflow-auto
    peer-data-[state=collapsed]:md:ml-[--sidebar-width-icon]
    peer-data-[state=expanded]:md:ml-[--sidebar-width]
    transition-[margin] duration-200"
>
  {@render children?.()}
</main>
