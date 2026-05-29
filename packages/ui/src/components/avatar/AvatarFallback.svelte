<script lang="ts">
import { Avatar } from "@ark-ui/svelte/avatar";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Avatar.Fallback data-slot="avatar-fallback" class={cn("bg-muted text-muted-foreground flex size-full items-center justify-center rounded-full text-sm", className)} {...rest}>
  {@render children?.()}
</Avatar.Fallback>
