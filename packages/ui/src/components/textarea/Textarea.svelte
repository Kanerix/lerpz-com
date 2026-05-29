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
    "border-input bg-input/30 placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 flex w-full rounded-2xl border px-3 py-2 text-sm transition-colors focus-visible:ring-[3px] focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 resize-none",
    className
  )}
  {...rest}
></textarea>
