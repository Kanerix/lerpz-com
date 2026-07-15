<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";

// The submit/send button shared by prompt composers. Shows a spinner while
// busy and an upward arrow otherwise.
let {
    loading = false,
    disabled = false,
    label = "Send",
    onclick,
}: {
    loading?: boolean;
    disabled?: boolean;
    /** Accessible label describing the action (e.g. "Send prompt"). */
    label?: string;
    onclick?: () => void;
} = $props();
</script>

<Button variant="ghost" size="icon" {disabled} {onclick} aria-label={label}>
  {#if loading}
    <Icon icon="fa6-solid:spinner" class="animate-spin" />
  {:else}
    <Icon icon="fa6-solid:arrow-up" />
  {/if}
</Button>
