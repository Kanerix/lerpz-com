<script lang="ts">
import type { HTMLInputAttributes } from "svelte/elements";
import { cn } from "../../lib/utils.js";

let { class: className = "", ...rest }: HTMLInputAttributes = $props();
</script>

<input
  data-slot="input"
  class={cn(
    "h-9 w-full px-3 py-2 text-sm",
    "bg-input/30 rounded-4xl border border-input transition-colors",
    "placeholder:text-muted-foreground",
    "focus-visible:border-ring focus-visible:ring-ring/50",
    "focus-visible:ring-[3px] focus-visible:outline-none",
    "disabled:cursor-not-allowed disabled:opacity-50",
    className
  )}
  {...rest}
/>
