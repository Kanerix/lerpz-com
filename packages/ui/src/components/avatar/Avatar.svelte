<script lang="ts">
import { Avatar } from "@ark-ui/svelte/avatar";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    size = "default",
    children,
}: {
    class?: string;
    size?: "sm" | "default" | "lg";
    children?: Snippet;
} = $props();
</script>

<Avatar.Root
  data-slot="avatar"
  data-size={size}
  class={cn(
    "relative flex shrink-0 select-none overflow-hidden rounded-full",
    size === "sm" && "size-6",
    size === "default" && "size-8",
    size === "lg" && "size-10",
    className
  )}
>
  {@render children?.()}
</Avatar.Root>
