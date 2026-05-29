<script lang="ts">
import { cn } from "../../lib/utils.js";
import { useSidebar } from "./context.svelte.js";

let {
    class: className = "",
    ...rest
}: { class?: string; [key: string]: unknown } = $props();
const sidebar = useSidebar();
</script>

<button
  data-slot="sidebar-trigger"
  type="button"
  onclick={() => sidebar.toggleSidebar()}
  class={cn("inline-flex size-7 items-center justify-center rounded-md text-sidebar-foreground hover:bg-sidebar-accent", className)}
  aria-label="Toggle sidebar"
  {...rest}
>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-4">
    <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
  </svg>
</button>
