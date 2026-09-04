<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { cn } from "@lerpz/ui/lib/utils";

let {
    title = "Something went wrong",
    description = "Please try again.",
    icon = "fa6-solid:triangle-exclamation",
    onRetry,
    retrying = false,
    retryLabel = "Try again",
    compact = false,
    class: className,
}: {
    /** Short headline describing what failed. */
    title?: string;
    /** Supporting line beneath the title. Pass an empty string to hide it. */
    description?: string;
    /** Icon shown in the badge. */
    icon?: string;
    /** When provided, renders a retry button that calls this on click. */
    onRetry?: () => void;
    /** Shows a spinner and "Retrying…" while a retry is in flight. */
    retrying?: boolean;
    /** Label for the retry button when idle. */
    retryLabel?: string;
    /** Tighter spacing for dense surfaces like the sidebar. */
    compact?: boolean;
    /** Extra classes for the wrapper (e.g. centering/max-width). */
    class?: string;
} = $props();
</script>

<div
  class={cn(
    "flex flex-col items-center text-center",
    compact ? "gap-2 px-2 py-4" : "gap-3 px-4 py-16",
    className,
  )}
>
  <div
    class={cn(
      "flex items-center justify-center rounded-full bg-destructive/10 text-destructive",
      compact ? "size-9" : "size-11",
    )}
  >
    <Icon {icon} class={compact ? "size-4" : "size-5"} />
  </div>
  <p class="text-sm font-medium">{title}</p>
  {#if description}
    <p class="text-muted-foreground text-xs text-balance">{description}</p>
  {/if}
  {#if onRetry}
    <Button
      variant="outline"
      size="sm"
      class="mt-1"
      disabled={retrying}
      onclick={onRetry}
    >
      <Icon
        icon="fa6-solid:arrow-rotate-right"
        class={retrying ? "animate-spin" : ""}
      />
      {retrying ? "Retrying…" : retryLabel}
    </Button>
  {/if}
</div>
