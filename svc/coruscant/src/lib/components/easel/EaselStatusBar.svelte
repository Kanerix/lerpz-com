<script lang="ts">
import Icon from "@iconify/svelte";
import { fade } from "svelte/transition";

let {
    isGenerating = false,
    error = null,
}: {
    isGenerating?: boolean;
    error?: string | null;
} = $props();

const quotes = [
    "Mixing the pigments…",
    "Priming the canvas…",
    "Sketching the composition…",
    "Chasing the light…",
    "Blending the palette…",
    "Rendering the details…",
    "Dreaming in pixels…",
    "Adjusting the brushstrokes…",
    "Summoning the muse…",
    "Developing the film…",
];

let quoteIndex = $state(0);

$effect(() => {
    if (!isGenerating) return;
    quoteIndex = Math.floor(Math.random() * quotes.length);
    const interval = setInterval(() => {
        quoteIndex = (quoteIndex + 1) % quotes.length;
    }, 2600);
    return () => clearInterval(interval);
});

type Status = "error" | "generating" | "idle";
const status = $derived<Status>(
    error ? "error" : isGenerating ? "generating" : "idle",
);
</script>

<div class="pointer-events-none mb-1 flex min-h-5 items-center justify-center px-4">
  {#if status === "error"}
    <div
      in:fade={{ duration: 120 }}
      out:fade={{ duration: 120 }}
      role="alert"
      class="pointer-events-auto flex max-w-full items-center gap-1.5 rounded-full border border-destructive/30 bg-destructive/10 px-3 py-1 text-xs font-medium text-destructive shadow-sm"
    >
      <Icon icon="fa6-solid:circle-exclamation" class="size-3.5 shrink-0" />
      <span class="truncate">{error}</span>
    </div>
  {:else if status === "generating"}
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
  {/if}
</div>
