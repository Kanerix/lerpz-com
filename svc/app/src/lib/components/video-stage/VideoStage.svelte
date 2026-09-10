<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { cn } from "@lerpz/ui/lib/utils";
import { resolveAspectRatio } from "$lib/components/clapper/clapper.store.svelte.js";
import { fade, scale } from "$lib/utils/transitions.js";

let {
    video = null,
    isLoading = false,
    isBackgrounded = false,
    startedAt = null,
    error = null,
    aspectRatio = "16:9",
    onRetry,
    onDismiss,
    onBackground,
    onForeground,
    class: className = "",
}: {
    /** Ready-to-play URL of the finished render, if any. */
    video?: string | null;
    /** Whether a render is currently in flight. */
    isLoading?: boolean;
    /** Whether the in-flight render was pushed to the background. */
    isBackgrounded?: boolean;
    /**
     * Epoch ms the render started. Owned by the job rather than this
     * component, so the elapsed timer keeps counting across page visits.
     */
    startedAt?: number | null;
    /** Failure message from a terminal job, if any. */
    error?: string | null;
    /** Selected aspect ratio, used to size the loading placeholder. */
    aspectRatio?: string;
    /** Re-run the last generation. */
    onRetry?: () => void;
    /** Clear the current result/error back to the idle state. */
    onDismiss?: () => void;
    /** Let the render finish out of sight, keeping the stage free. */
    onBackground?: () => void;
    /** Bring a backgrounded render's progress back into view. */
    onForeground?: () => void;
    class?: string;
} = $props();

// Size the placeholder to match what the user will actually get back. The box
// is fitted to the stage with container units (see `.stage-frame`), so it
// letterboxes instead of pushing the page into an awkward scroll.
const ratio = $derived(resolveAspectRatio(aspectRatio));

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
    // Fall back to now if the caller doesn't track a start time, so the timer
    // still counts (just from this mount) rather than sitting at zero.
    const started = startedAt ?? Date.now();
    const update = () => {
        elapsed = Math.max(0, Math.floor((Date.now() - started) / 1000));
    };
    update();
    const tick = setInterval(update, 1000);
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

const showProgress = $derived(isLoading && !isBackgrounded);
</script>

<div
  class={cn("flex h-full min-h-0 w-full flex-col overflow-hidden", className)}
>
  <!-- The stage itself: sized to the space left over, never taller. Children
       centre themselves with `m-auto` so that, on very short viewports, they
       scroll fully into reach instead of being clipped at the top. -->
  <div
    class="stage-viewport relative flex min-h-0 w-full flex-1 items-center justify-center overflow-y-auto"
  >
    {#if showProgress}
      <!-- Generating: aspect-correct shimmer with progress affordances. -->
      <div
        in:scale={{ duration: 200, start: 0.98 }}
        class="stage-frame relative flex flex-col items-center justify-center gap-4 overflow-hidden rounded-2xl border bg-muted/40 p-4 shadow-lg"
        style="--ar-w:{ratio.w};--ar-h:{ratio.h}"
      >
        <div class="shimmer pointer-events-none absolute inset-0"></div>
        <div
          class="relative flex size-14 shrink-0 items-center justify-center rounded-full bg-background/70 text-primary shadow-sm backdrop-blur"
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
        <div class="relative flex shrink-0 flex-wrap items-center justify-center gap-2">
          <span
            class="flex items-center gap-1.5 rounded-full border bg-background/70 px-2.5 py-1 font-mono text-xs text-muted-foreground backdrop-blur"
          >
            <Icon icon="fa6-regular:clock" class="size-3" />
            {elapsedLabel}
          </span>
          {#if onBackground}
            <Button
              variant="outline"
              size="sm"
              class="rounded-full bg-background/70 backdrop-blur"
              onclick={onBackground}
              title="Keep rendering and notify me when it's done"
            >
              <Icon icon="fa6-solid:down-left-and-up-right-to-center" class="size-3.5" />
              Run in background
            </Button>
          {/if}
        </div>
      </div>
    {:else if isLoading}
      <!-- Backgrounded: a quiet placeholder while the render finishes out of
           sight; the notification bell reports the outcome. -->
      <div
        in:fade={{ duration: 200 }}
        class="m-auto flex flex-col items-center gap-3 px-4 text-center"
      >
        <span
          class="relative flex size-12 shrink-0 items-center justify-center rounded-full bg-muted text-muted-foreground/80"
        >
          <Icon icon="fa6-solid:film" class="size-5" />
          <Icon
            icon="fa6-solid:spinner"
            class="absolute -right-1 -bottom-1 size-4 animate-spin rounded-full bg-background p-0.5 text-muted-foreground"
          />
        </span>
        <div class="space-y-1">
          <p class="text-base font-medium">Rendering in the background</p>
          <p class="max-w-sm text-sm text-muted-foreground">
            You'll get a notification when the video is ready — it keeps going
            even if you leave this page.
          </p>
        </div>
        <div class="flex items-center gap-2">
          <span
            class="flex items-center gap-1.5 rounded-full border bg-muted/40 px-2.5 py-1 font-mono text-xs text-muted-foreground"
          >
            <Icon icon="fa6-regular:clock" class="size-3" />
            {elapsedLabel}
          </span>
          {#if onForeground}
            <Button variant="outline" size="sm" onclick={onForeground}>
              <Icon
                icon="fa6-solid:up-right-and-down-left-from-center"
                class="size-3.5"
              />
              Show progress
            </Button>
          {/if}
        </div>
      </div>
    {:else if error}
      <!-- Failed: surface the reason and offer a retry. -->
      <div
        in:scale={{ duration: 200, start: 0.98 }}
        class="m-auto flex w-full max-w-md flex-col items-center gap-4 rounded-2xl border border-destructive/30 bg-destructive/5 p-8 text-center"
      >
        <span
          class="flex size-12 shrink-0 items-center justify-center rounded-full bg-destructive/10 text-destructive"
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
        class="flex h-full min-h-0 w-full flex-col items-center justify-center gap-3"
      >
        <div class="flex min-h-0 w-full flex-1 items-center justify-center">
          <!-- svelte-ignore a11y_media_has_caption -->
          <video
            src={video}
            controls
            autoplay
            loop
            class="max-h-full max-w-full rounded-2xl object-contain shadow-lg"
          ></video>
        </div>
        <div class="flex shrink-0 items-center gap-2">
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
        class="m-auto flex flex-col items-center gap-3 px-4 text-center text-muted-foreground"
      >
        <span
          class="flex size-12 shrink-0 items-center justify-center rounded-full bg-muted text-muted-foreground/80"
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
</div>

<style>
.stage-viewport {
	container-type: size;
}

.stage-frame {
	width: min(100cqw, calc(100cqh * var(--ar-w) / var(--ar-h)));
	aspect-ratio: var(--ar-w) / var(--ar-h);
	max-width: 100%;
	max-height: 100%;
}

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
