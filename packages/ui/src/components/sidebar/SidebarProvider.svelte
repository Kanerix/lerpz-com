<script lang="ts">
import type { Snippet } from "svelte";
import { SidebarContext, setSidebarContext } from "./context.svelte.js";

let { children }: { children?: Snippet } = $props();

const ctx = new SidebarContext();
setSidebarContext(ctx);

// Track mobile viewport
$effect(() => {
    const mq = window.matchMedia("(max-width: 768px)");
    ctx.isMobile = mq.matches;
    const handler = (e: MediaQueryListEvent) => {
        ctx.isMobile = e.matches;
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
});
</script>

<div
  data-slot="sidebar-provider"
  data-state={ctx.state}
  style="--sidebar-width: 24rem; --sidebar-width-icon: 3rem;"
  class="group/sidebar-wrapper flex min-h-screen w-full"
>
  {@render children?.()}
</div>
