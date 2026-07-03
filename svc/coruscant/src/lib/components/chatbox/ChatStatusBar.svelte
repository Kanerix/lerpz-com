<script lang="ts">
import Icon from "@iconify/svelte";
import { fade } from "svelte/transition";

let {
    isThinking = false,
    isSaved = false,
    error = null,
}: {
    isThinking?: boolean;
    isSaved?: boolean;
    error?: string | null;
} = $props();

const quotes = [
    "Thinking really hard…",
    "Consulting the rubber duck…",
    "Reticulating splines…",
    "Summoning electrons…",
    "Untangling the cables…",
    "Asking the magic conch…",
    "Borrowing a cup of RAM…",
    "Counting to infinity…",
    "Polishing the pixels…",
    "Bribing the hamsters…",
    "Aligning the flux capacitor…",
    "Pretending to read the docs…",
];

let quoteIndex = $state(0);

$effect(() => {
    if (!isThinking) return;
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
    if (isThinking) sawActivity = true;
});

$effect(() => {
    const saved = isSaved;
    const isFreshSave = saved && !prevSaved && sawActivity;
    prevSaved = saved;
    if (!isFreshSave) return;
    sawActivity = false;
    showSaved = true;
    const timeout = setTimeout(() => {
        showSaved = false;
    }, 2500);
    return () => clearTimeout(timeout);
});

type Status = "error" | "thinking" | "saved" | "idle";
const status = $derived<Status>(
    error ? "error" : isThinking ? "thinking" : showSaved ? "saved" : "idle",
);
</script>

<div class="pointer-events-none flex min-h-5 items-center justify-center px-4 mb-1 bg-opacity-0">
  {#if status === "error"}
    <div
      in:fade={{ duration: 120 }}
      out:fade={{ duration: 120 }}
      role="alert"
      class="pointer-events-auto flex max-w-full items-center gap-1.5 rounded-full bg-destructive px-3 py-1 text-xs font-medium text-destructive-foreground shadow-sm"
    >
      <Icon icon="fa6-solid:circle-exclamation" class="size-3.5 shrink-0" />
      <span class="truncate">{error}</span>
    </div>
  {:else if status === "thinking"}
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
