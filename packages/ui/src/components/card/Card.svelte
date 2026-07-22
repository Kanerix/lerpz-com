<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<div
  data-slot="card"
  class={cn(
    "bg-card text-card-foreground rounded-lg border shadow-sm",
    className
  )}
  {...rest}
>
  {@render children?.()}
</div>
