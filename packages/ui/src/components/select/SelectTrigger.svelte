<script lang="ts">
import { Select } from "@ark-ui/svelte/select";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    size = "default",
    children,
}: { class?: string; size?: "sm" | "default"; children?: Snippet } = $props();
</script>

<Select.Control>
  <Select.Trigger
    data-slot="select-trigger"
    class={cn(
      "flex w-fit items-center justify-between gap-1.5 px-3 text-sm",
      "bg-input/30 rounded-lg border border-input transition-colors",
      "cursor-pointer",
      "focus-visible:border-ring focus-visible:ring-ring/50",
      "focus-visible:ring-[3px] focus-visible:outline-none",
      "disabled:cursor-not-allowed disabled:opacity-50",
      size === "default" ? "h-9" : "h-8",
      className
    )}
  >
    {@render children?.()}
    <Select.Indicator>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="currentColor"
        class="size-4 text-muted-foreground"
      >
        <path d="M7 10l5 5 5-5H7z"/>
      </svg>
    </Select.Indicator>
  </Select.Trigger>
</Select.Control>
