<script lang="ts">
import { Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    value,
    children,
}: { class?: string; value: string; children?: Snippet } = $props();

const item = $derived({ value, label: "" });
</script>

<Select.Item
  data-slot="select-item"
  {item}
  class={cn(
    "focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground relative flex w-full cursor-pointer select-none items-center gap-2 rounded-xl py-2 pr-8 pl-3 text-sm outline-none",
    "data-disabled:pointer-events-none data-disabled:opacity-50",
    className
  )}
>
  <Select.ItemText>{@render children?.()}</Select.ItemText>
  <Select.ItemIndicator class="pointer-events-none absolute right-2 flex size-4 items-center justify-center">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-3.5">
      <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"/>
    </svg>
  </Select.ItemIndicator>
</Select.Item>
