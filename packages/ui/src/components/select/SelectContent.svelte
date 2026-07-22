<script lang="ts">
import { Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Select.Positioner class="isolate z-50">
  <Select.Content
    data-slot="select-content"
    class={cn(
      "z-50 min-w-36 p-1 bg-popover text-popover-foreground",
      "rounded-lg border shadow-2xl outline-none",
      "data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
      "data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      className
    )}
    {...rest}
  >
    <Select.List>
      {@render children?.()}
    </Select.List>
  </Select.Content>
</Select.Positioner>
