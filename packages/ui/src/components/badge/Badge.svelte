<script lang="ts">
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";
import { type BadgeVariants, badgeVariants } from "./badge-variants.js";

let {
    class: className = "",
    variant = "default",
    children,
    ...rest
}: {
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
} & BadgeVariants = $props();
</script>

<span
  data-slot="badge"
  class={cn(badgeVariants({ variant, className }))}
  {...rest}
>
  {@render children?.()}
</span>
