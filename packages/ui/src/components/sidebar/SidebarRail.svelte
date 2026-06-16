<script lang="ts">
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";

const sidebar = useSidebar();
</script>
<button
  data-slot="sidebar-rail"
  type="button"
  aria-label="Toggle sidebar"
  onclick={() => sidebar.toggleSidebar()}
  class={cn(
    "absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 cursor-pointer",
    "transition-all ease-linear",
    "after:absolute after:inset-y-0 after:left-1/2 after:w-[2px]",
    "hover:after:bg-sidebar-border sm:flex"
  )}
></button>
