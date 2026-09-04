<script lang="ts">
import Icon from "@iconify/svelte";
import { cn } from "@lerpz/ui/lib/utils";
import { fade } from "svelte/transition";

// A compact status line shown above a prompt composer. Rotates through
// playful "busy" messages while working, surfaces errors, and can flash a
// transient "Saved" confirmation.
let {
    busy = false,
    quotes = [],
    error = null,
    onShowError,
    saved = false,
    class: className = "",
}: {
    /** Whether a long-running task (thinking/generating) is in progress. */
    busy?: boolean;
    /** Playful messages cycled through while busy. */
    quotes?: string[];
    error?: string | null;
    /**
     * When provided, the error renders as a clickable button (labelled
     * "Show error"). Otherwise the raw error message is shown inline.
     */
    onShowError?: () => void;
    /** Flash a transient "Saved" confirmation after activity settles. */
    saved?: boolean;
    class?: string;
} = $props();

let quoteIndex = $state(0);

$effect(() => {
    if (!busy || quotes.length === 0) return;
    quoteIndex = Math.floor(Math.random() * quotes.length);
    const interval = setInterval(() => {
        quoteIndex = (quoteIndex + 1) % quotes.length;
    }, 2600);
    return () => clearInterval(interval);
});

let showSaved = $state(false);
let sawActivity = false;
let prevSaved = false;

$effect(() => {
    if (busy) sawActivity = true;
});

$effect(() => {
    const isSaved = saved;
    const isFreshSave = isSaved && !prevSaved && sawActivity;
    prevSaved = isSaved;
    if (!isFreshSave) return;
    sawActivity = false;
    showSaved = true;
    const timeout = setTimeout(() => {
        showSaved = false;
    }, 2500);
    return () => clearTimeout(timeout);
});

type Status = "error" | "busy" | "saved" | "idle";
const status = $derived<Status>(
    error ? "error" : busy ? "busy" : showSaved ? "saved" : "idle",
);
</script>

<div
  class={cn(
    "pointer-events-none mb-1 flex min-h-5 items-center justify-center px-4",
    className,
  )}
>
  {#if status === "error"}
    {#if onShowError}
      <button
        type="button"
        onclick={() => onShowError?.()}
        in:fade={{ duration: 120 }}
        out:fade={{ duration: 120 }}
        class="pointer-events-auto flex max-w-full cursor-pointer items-center gap-1.5 rounded-full border border-destructive/30 bg-destructive/10 px-3 py-1 text-xs font-medium text-destructive shadow-sm transition-colors hover:bg-destructive/20"
      >
        <Icon icon="fa6-solid:circle-exclamation" class="size-3.5 shrink-0" />
        <span>Show error</span>
      </button>
    {:else}
      <div
        in:fade={{ duration: 120 }}
        out:fade={{ duration: 120 }}
        role="alert"
        class="pointer-events-auto flex max-w-full items-center gap-1.5 rounded-full border border-destructive/30 bg-destructive/10 px-3 py-1 text-xs font-medium text-destructive shadow-sm"
      >
        <Icon icon="fa6-solid:circle-exclamation" class="size-3.5 shrink-0" />
        <span class="truncate">{error}</span>
      </div>
    {/if}
  {:else if status === "busy"}
    <div
      in:fade={{ duration: 150 }}
      out:fade={{ duration: 150 }}
      class="flex items-center gap-1.5 text-xs text-muted-foreground/70"
    >
      <Icon icon="fa6-solid:spinner" class="size-3 shrink-0 animate-spin" />
      {#key quoteIndex}
        <span in:fade={{ duration: 250 }}>{quotes[quoteIndex]}</span>
      {/key}
    </div>
  {:else if status === "saved"}
    <div
      in:fade={{ duration: 150 }}
      out:fade={{ duration: 300 }}
      class="flex items-center gap-1.5 text-xs text-muted-foreground/70"
    >
      <Icon icon="fa6-regular:circle-check" class="size-3 shrink-0" />
      <span>Saved</span>
    </div>
  {/if}
</div>
