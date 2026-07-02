<script lang="ts">
import { Dialog } from "@ark-ui/svelte/dialog";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

let {
    class: className = "",
    children,
    ...rest
}: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<Dialog.Title
  data-slot="dialog-title"
  class={cn("text-lg font-semibold leading-none", className)}
  {...rest}
>
  {@render children?.()}
</Dialog.Title>
