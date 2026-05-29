<script lang="ts">
import type { HTMLInputAttributes } from "svelte/elements";
import { cn } from "../../lib/utils.js";

let { class: className = "", ...rest }: HTMLInputAttributes = $props();
</script>

<input
  data-slot="input"
  class={cn(
    "border-input bg-input/30 placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 h-9 w-full rounded-4xl border px-3 py-2 text-sm transition-colors focus-visible:ring-[3px] focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50",
    className
  )}
  {...rest}
/>
