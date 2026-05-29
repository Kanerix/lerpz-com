<script lang="ts">
import { useSidebar } from "./context.svelte.js";

const sidebar = useSidebar();
</script>
<button
  data-slot="sidebar-rail"
  type="button"
  aria-label="Toggle sidebar"
  onclick={() => sidebar.toggleSidebar()}
  class="hover:after:bg-sidebar-border absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] sm:flex"
></button>
