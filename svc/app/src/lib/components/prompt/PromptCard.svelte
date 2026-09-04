<script lang="ts">
import { Card, CardContent } from "@lerpz/ui/components/card";
import { cn } from "@lerpz/ui/lib/utils";
import type { Snippet } from "svelte";

// Shared shell for prompt composers (chat + image generation) so their
// width, radius, background and padding stay identical.
let {
    class: className = "",
    children,
}: { class?: string; children?: Snippet } = $props();
</script>

<Card
  class={cn(
    "rounded-4xl border-sidebar-border bg-sidebar text-sidebar-foreground",
    className,
  )}
>
  <CardContent class="flex flex-col gap-2 p-4">
    {@render children?.()}
  </CardContent>
</Card>
