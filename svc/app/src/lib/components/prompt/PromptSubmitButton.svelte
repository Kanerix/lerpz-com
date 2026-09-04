<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";

// The submit/send button shared by prompt composers. While a reply is
// streaming it turns into a clickable stop square so the user can cancel;
// otherwise it shows a spinner when busy and an upward arrow when idle.
let {
    loading = false,
    streaming = false,
    disabled = false,
    label = "Send",
    stopLabel = "Stop generating",
    onclick,
    onStop,
}: {
    loading?: boolean;
    /** When true, show a clickable stop square that calls `onStop`. */
    streaming?: boolean;
    disabled?: boolean;
    /** Accessible label describing the send action (e.g. "Send prompt"). */
    label?: string;
    /** Accessible label describing the stop action. */
    stopLabel?: string;
    onclick?: () => void;
    /** Called when the user clicks the stop square while streaming. */
    onStop?: () => void;
} = $props();
</script>

{#if streaming}
  <Button
    variant="destructive"
    size="icon"
    onclick={onStop}
    aria-label={stopLabel}
  >
    <Icon icon="fa6-solid:stop" />
  </Button>
{:else}
  <Button variant="ghost" size="icon" {disabled} {onclick} aria-label={label}>
    {#if loading}
      <Icon icon="fa6-solid:spinner" class="animate-spin" />
    {:else}
      <Icon icon="fa6-solid:arrow-up" />
    {/if}
  </Button>
{/if}
