<script lang="ts">
import type { HTMLTextareaAttributes } from "svelte/elements";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    rows = 3,
    el = $bindable<HTMLTextAreaElement | null>(null),
    ...rest
}: HTMLTextareaAttributes & { el?: HTMLTextAreaElement | null } = $props();
</script>
<textarea
  data-slot="textarea"
  {rows}
  bind:this={el}
  class={cn(
    "flex w-full px-3 py-2 text-sm",
    "bg-input/30 rounded-2xl border border-input transition-colors resize-none",
    "placeholder:text-muted-foreground",
    "focus-visible:border-ring focus-visible:ring-ring/50",
    "focus-visible:ring-[3px] focus-visible:outline-none",
    "disabled:cursor-not-allowed disabled:opacity-50",
    className
  )}
  {...rest}
></textarea>
