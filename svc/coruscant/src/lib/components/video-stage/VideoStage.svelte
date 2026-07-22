<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { cn } from "@lerpz/ui/lib/utils";
import { fade, scale } from "svelte/transition";
import { resolveAspectRatio } from "$lib/components/clapper/clapper.store.svelte.js";

let {
    video = null,
    isLoading = false,
    error = null,
    aspectRatio = "16:9",
    onRetry,
    onDismiss,
    class: className = "",
}: {
    /** Ready-to-play URL of the finished render, if any. */
    video?: string | null;
    /** Whether a render is currently in flight. */
    isLoading?: boolean;
    /** Failure message from a terminal job, if any. */
    error?: string | null;
    /** Selected aspect ratio, used to size the loading placeholder. */
    aspectRatio?: string;
    /** Re-run the last generation. */
    onRetry?: () => void;
    /** Clear the current result/error back to the idle state. */
    onDismiss?: () => void;
    class?: string;
} = $props();

// Size the placeholder to match what the user will actually get back.
const ratio = $derived(resolveAspectRatio(aspectRatio));
const isPortrait = $derived(ratio.h > ratio.w);
const stageStyle = $derived(
    isPortrait
        ? `aspect-ratio:${ratio.w}/${ratio.h};height:min(70vh,100%);max-width:100%;`
        : `aspect-ratio:${ratio.w}/${ratio.h};width:100%;max-width:48rem;max-height:70vh;`,
);

// A little director's-cut flavour text that cycles while rendering.
const quotes = [
    "Storyboarding the scene…",
    "Rolling the camera…",
    "Rendering the frames…",
    "Interpolating motion…",
    "Lighting the set…",
    "Cutting the reel…",
];

let elapsed = $state(0);
let quoteIndex = $state(0);

// Drive the elapsed timer and rotating status only while a render is active.
$effect(() => {
    if (!isLoading) {
        elapsed = 0;
        quoteIndex = 0;
        return;
    }
    const started = Date.now();
    const tick = setInterval(() => {
        elapsed = Math.floor((Date.now() - started) / 1000);
    }, 1000);
    const rotate = setInterval(() => {
        quoteIndex = (quoteIndex + 1) % quotes.length;
    }, 3500);
    return () => {
        clearInterval(tick);
        clearInterval(rotate);
    };
});

const elapsedLabel = $derived.by(() => {
    const m = Math.floor(elapsed / 60);
    const s = elapsed % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
});
</script>

<div class={cn("flex h-full w-full items-center justify-center p-2", className)}>
  {#if isLoading}
    <!-- Generating: aspect-correct shimmer with progress affordances. -->
    <div
      in:scale={{ duration: 200, start: 0.98 }}
      class="relative flex flex-col items-center justify-center gap-4 overflow-hidden rounded-2xl border bg-muted/40 shadow-lg"
      style={stageStyle}
    >
      <div class="shimmer pointer-events-none absolute inset-0"></div>
      <div
        class="relative flex size-14 items-center justify-center rounded-full bg-background/70 text-primary shadow-sm backdrop-blur"
      >
        <Icon icon="fa6-solid:film" class="size-6" />
        <Icon
          icon="fa6-solid:spinner"
          class="absolute -right-1 -bottom-1 size-5 animate-spin rounded-full bg-background p-0.5 text-muted-foreground"
        />
      </div>
      <div class="relative flex flex-col items-center gap-1 px-6 text-center">
        <p class="text-sm font-medium">Generating your video</p>
        {#key quoteIndex}
          <p in:fade={{ duration: 300 }} class="text-xs text-muted-foreground">
            {quotes[quoteIndex]}
          </p>
        {/key}
      </div>
      <div
        class="relative flex items-center gap-1.5 rounded-full border bg-background/70 px-2.5 py-1 font-mono text-xs text-muted-foreground backdrop-blur"
      >
        <Icon icon="fa6-regular:clock" class="size-3" />
        {elapsedLabel}
      </div>
    </div>
  {:else if error}
    <!-- Failed: surface the reason and offer a retry. -->
    <div
      in:scale={{ duration: 200, start: 0.98 }}
      class="flex w-full max-w-md flex-col items-center gap-4 rounded-2xl border border-destructive/30 bg-destructive/5 p-8 text-center"
    >
      <span
        class="flex size-12 items-center justify-center rounded-full bg-destructive/10 text-destructive"
      >
        <Icon icon="fa6-solid:triangle-exclamation" class="size-5" />
      </span>
      <div class="space-y-1">
        <p class="text-base font-medium">Generation failed</p>
        <p class="text-sm text-muted-foreground">{error}</p>
      </div>
      <div class="flex items-center gap-2">
        {#if onRetry}
          <Button size="sm" onclick={onRetry}>
            <Icon icon="fa6-solid:rotate-right" class="size-3.5" />
            Try again
          </Button>
        {/if}
        {#if onDismiss}
          <Button variant="ghost" size="sm" onclick={onDismiss}>Dismiss</Button>
        {/if}
      </div>
    </div>
  {:else if video}
    <!-- Completed: the finished render with quick actions. -->
    <div
      in:scale={{ duration: 200, start: 0.98 }}
      class="flex h-full w-full flex-col items-center justify-center gap-3"
    >
      <!-- svelte-ignore a11y_media_has_caption -->
      <video
        src={video}
        controls
        autoplay
        loop
        class="max-h-[70vh] max-w-full rounded-2xl object-contain shadow-lg"
      ></video>
      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" href={video} download="video.mp4">
          <Icon icon="fa6-solid:download" class="size-3.5" />
          Download
        </Button>
        {#if onDismiss}
          <Button variant="ghost" size="sm" onclick={onDismiss}>
            <Icon icon="fa6-solid:xmark" class="size-3.5" />
            Clear
          </Button>
        {/if}
      </div>
    </div>
  {:else}
    <!-- Idle: nothing generated yet. -->
    <div
      in:fade={{ duration: 200 }}
      class="flex flex-col items-center gap-3 text-center text-muted-foreground"
    >
      <span
        class="flex size-12 items-center justify-center rounded-full bg-muted text-muted-foreground/80"
      >
        <Icon icon="fa6-solid:clapperboard" class="size-5" />
      </span>
      <div class="space-y-1">
        <p class="text-base font-medium text-foreground">
          Describe a video to bring it to life
        </p>
        <p class="max-w-sm text-sm">
          Write a prompt below, pick a model, aspect ratio and duration, then
          generate.
        </p>
      </div>
    </div>
  {/if}
</div>

<style>
.shimmer {
    background: linear-gradient(
        105deg,
        transparent 30%,
        color-mix(in oklab, var(--foreground) 8%, transparent) 50%,
        transparent 70%
    );
    background-size: 200% 100%;
    animation: shimmer 1.8s ease-in-out infinite;
}

@keyframes shimmer {
    from {
        background-position: 200% 0;
    }
    to {
        background-position: -200% 0;
    }
}
</style>
