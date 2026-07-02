<script lang="ts">
import { Dialog } from "@ark-ui/svelte/dialog";
import type { Snippet } from "svelte";

let {
    open = $bindable(),
    children,
    ...rest
}: {
    open?: boolean;
    children?: Snippet;
    [key: string]: unknown;
} = $props();
</script>

<Dialog.Root bind:open {...rest}>
  {@render children?.()}
</Dialog.Root>
