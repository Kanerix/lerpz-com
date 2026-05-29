<script lang="ts">
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    orientation = "horizontal",
    ...rest
}: {
    class?: string;
    orientation?: "horizontal" | "vertical";
    [key: string]: unknown;
} = $props();
</script>
<div
  data-slot="separator"
  data-orientation={orientation}
  class={cn(
    "bg-border shrink-0",
    orientation === "horizontal" ? "h-px w-full" : "h-full w-px",
    className
  )}
  {...rest}
></div>
