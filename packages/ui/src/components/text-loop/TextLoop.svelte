<script lang="ts">
import { fly } from "svelte/transition";
import { cn } from "../../lib/utils.js";

let {
    items,
    class: className = "",
    interval = 2,
}: {
    items: string[];
    class?: string;
    interval?: number;
} = $props();

let currentIndex = $state(0);

$effect(() => {
    const ms = interval * 1000;
    const timer = setInterval(() => {
        currentIndex = (currentIndex + 1) % items.length;
    }, ms);
    return () => clearInterval(timer);
});
</script>

<div class={cn("relative inline-grid whitespace-nowrap", className)}>
  {#key currentIndex}
    <span
      class="col-start-1 row-start-1"
      in:fly={{ y: 20, duration: 300 }}
      out:fly={{ y: -20, duration: 300 }}
      style="display: inline-block;"
    >
      {items[currentIndex]}
    </span>
  {/key}
</div>
