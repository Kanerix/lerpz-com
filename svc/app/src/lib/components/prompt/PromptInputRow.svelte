<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import type { Snippet } from "svelte";

// The input row shared by prompt composers: the text input followed by a
// trailing action (typically the submit button). Keeps alignment/spacing
// identical between chat and image generation.
let {
    class: className = "",
    children,
}: { class?: string; children?: Snippet } = $props();
</script>

<div class={cn("flex items-end gap-4", className)}>
  {@render children?.()}
</div>
