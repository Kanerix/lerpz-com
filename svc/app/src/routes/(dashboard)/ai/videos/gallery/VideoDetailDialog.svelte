<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Dialog,
    DialogBackdrop,
    DialogClose,
    DialogContent,
    DialogPositioner,
    DialogTitle,
} from "@lerpz/ui/components/dialog";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery } from "@tanstack/svelte-query";
import type { VideoItem, VideoListResponse } from "$lib/api/models/index.js";
import { getListVideosUrl, listVideos } from "$lib/api/videos/videos.js";
import { formatDate, formatDuration } from "$lib/utils/format.js";
import { fade, fly } from "$lib/utils/transitions.js";

// Number of surrounding videos to load for the "more videos" rail. Kept small
// so the dialog stays focused on the selected video.
const NEARBY_LIMIT = 6;

let {
    open = $bindable(false),
    video = null,
    onSelectVideo,
    onDownload,
    onOpenChange,
}: {
    /** Controls visibility of the dialog. Bindable. */
    open?: boolean;
    /** The video whose details are displayed. */
    video?: VideoItem | null;
    /** Called when a surrounding video is clicked, to swap the active video. */
    onSelectVideo?: (video: VideoItem) => void;
    /** Download the given video. */
    onDownload?: (video: VideoItem) => void;
    /** Called whenever the open state changes (backdrop click, Escape, …). */
    onOpenChange?: (open: boolean) => void;
} = $props();

// Surrounding videos: everything older than the current one, using its ID as
// the pagination cursor.
const nearbyQuery = createQuery(() => ({
    queryKey: [getListVideosUrl(), "nearby", video?.id],
    enabled: Boolean(video),
    queryFn: async ({
        signal,
    }: {
        signal: AbortSignal;
    }): Promise<VideoListResponse> => {
        const res = await listVideos(
            { cursor: video?.id, limit: NEARBY_LIMIT },
            { signal },
        );
        if (res.status !== 200) {
            throw new Error("Failed to load videos");
        }
        return res.data;
    },
}));

const nearbyVideos = $derived(nearbyQuery.data?.items ?? []);

const detailRows = $derived(
    video
        ? [
              { label: "Model", value: video.model, badge: true },
              {
                  label: "Dimensions",
                  value: `${video.width} × ${video.height}`,
                  badge: false,
              },
              {
                  label: "Duration",
                  value: formatDuration(video.duration),
                  badge: false,
              },
              {
                  label: "Format",
                  value: video.format.toUpperCase(),
                  badge: false,
              },
              {
                  label: "Created",
                  value: formatDate(video.created_at),
                  badge: false,
              },
          ]
        : [],
);

function hoverPlay(event: Event) {
    const el = event.currentTarget as HTMLVideoElement;
    el.play?.().catch(() => {});
}

function hoverPause(event: Event) {
    const el = event.currentTarget as HTMLVideoElement;
    el.pause();
    el.currentTime = 0;
}

function handleOpenChange(details: { open: boolean }) {
    open = details.open;
    onOpenChange?.(details.open);
}
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogBackdrop />
  <DialogPositioner>
    <DialogContent class="max-h-[88vh] w-full max-w-6xl overflow-hidden">
      {#if video}
        <div class="flex max-h-[88vh] flex-col overflow-hidden md:flex-row">
          <!-- Video -->
          <div
            class="relative flex min-h-64 flex-1 items-center justify-center overflow-hidden bg-linear-to-b from-muted/30 to-muted/60 p-4"
          >
            {#key video.id}
              <!-- svelte-ignore a11y_media_has_caption -->
              <video
                src={video.url}
                controls
                autoplay
                loop
                class="max-h-[80vh] w-auto max-w-full rounded-xl object-contain shadow-2xl"
                in:fly|global={{ y: 12, duration: 400 }}
              ></video>
            {/key}
          </div>

          <!-- Metadata -->
          <div
            class="flex w-full flex-col border-t md:w-80 md:shrink-0 md:border-t-0 md:border-l"
          >
            <div
              class="flex items-start gap-3 p-5 pb-3"
              in:fly|global={{ y: -8, duration: 300 }}
            >
              <div class="min-w-0 flex-1 space-y-1">
                {#key video.title}
                  <div in:fade|global={{ duration: 250 }}>
                    <DialogTitle class="truncate text-base">
                      {video.title || "Untitled"}
                    </DialogTitle>
                  </div>
                {/key}
                <p class="text-xs text-muted-foreground">Video details</p>
              </div>
              <DialogClose
                aria-label="Close"
                class="-mr-1 -mt-1 flex size-8 shrink-0 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
              >
                <Icon icon="fa6-solid:xmark" class="size-4" />
              </DialogClose>
            </div>

            {#key video.id}
              <!-- Prompt fills the leftover space and scrolls on its own. -->
              <div
                class="flex min-h-0 flex-1 flex-col gap-1.5 px-5"
                in:fly|global={{ y: 8, duration: 300, delay: 40 }}
              >
                <span class="text-xs font-medium text-muted-foreground">
                  Prompt
                </span>
                <ScrollArea orientation="vertical" class="flex-1">
                  <p class="pr-3 pb-1 text-sm leading-relaxed">
                    {video.prompt}
                  </p>
                </ScrollArea>
              </div>

              <!-- Metadata takes exactly the space it needs. -->
              <dl class="shrink-0 space-y-4 px-5 pt-4 pb-5 text-sm">
                {#each detailRows as row, i (row.label)}
                  <div
                    class="flex items-center justify-between gap-4"
                    in:fly|global={{ y: 8, duration: 300, delay: 80 + i * 40 }}
                  >
                    <dt class="text-muted-foreground">{row.label}</dt>
                    <dd class="min-w-0">
                      {#if row.badge}
                        <span
                          class="inline-flex max-w-full items-center truncate rounded-md bg-muted px-2 py-0.5 text-xs"
                        >
                          {row.value}
                        </span>
                      {:else}
                        <span class="font-medium">{row.value}</span>
                      {/if}
                    </dd>
                  </div>
                {/each}
              </dl>
            {/key}

            {#if onDownload}
              <div
                class="flex flex-col gap-2 border-t p-5"
                in:fly|global={{ y: 8, duration: 300, delay: 120 }}
              >
                <Button variant="outline" onclick={() => onDownload?.(video)}>
                  <Icon icon="fa6-solid:download" class="size-4" />
                  Download
                </Button>
              </div>
            {/if}
          </div>

          <!-- Surrounding videos -->
          <div
            class="hidden w-48 shrink-0 flex-col overflow-hidden border-l lg:flex"
          >
            <p class="px-4 pt-4 pb-2 text-xs font-medium text-muted-foreground">
              More videos
            </p>
            <ScrollArea orientation="vertical" class="flex-1">
              <div class="flex flex-col gap-2 px-4 pb-4">
                {#if nearbyQuery.isLoading}
                  {#each Array(NEARBY_LIMIT) as _, i (i)}
                    <Skeleton class="aspect-video w-full rounded-lg" />
                  {/each}
                {:else if nearbyQuery.isError}
                  <p class="text-xs text-destructive">
                    Failed to load more videos.
                  </p>
                {:else if nearbyVideos.length === 0}
                  <p class="text-xs text-muted-foreground">
                    No more videos to show.
                  </p>
                {:else}
                  {#each nearbyVideos as item, i (item.id)}
                    <button
                      type="button"
                      onclick={() => onSelectVideo?.(item)}
                      title={item.title ?? item.prompt}
                      class="group relative overflow-hidden rounded-lg border border-border bg-muted/30 outline-none transition hover:border-primary/50 hover:shadow-md focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
                      in:fly|global={{ y: 12, duration: 300, delay: i * 50 }}
                    >
                      <!-- svelte-ignore a11y_media_has_caption -->
                      <video
                        src={item.url}
                        muted
                        loop
                        playsinline
                        preload="metadata"
                        style="aspect-ratio: {item.width} / {item.height}"
                        onmouseenter={hoverPlay}
                        onmouseleave={hoverPause}
                        class="w-full object-cover transition-transform duration-300 group-hover:scale-[1.05]"
                      ></video>
                      <span
                        class="pointer-events-none absolute inset-0 flex items-center justify-center bg-black/10 opacity-100 transition-opacity group-hover:opacity-0"
                      >
                        <span
                          class="flex size-7 items-center justify-center rounded-full bg-black/45 text-white ring-1 ring-white/20 backdrop-blur-sm"
                        >
                          <Icon
                            icon="fa6-solid:play"
                            class="size-2.5 translate-x-px"
                          />
                        </span>
                      </span>
                    </button>
                  {/each}
                {/if}
              </div>
            </ScrollArea>
          </div>
        </div>
      {/if}
    </DialogContent>
  </DialogPositioner>
</Dialog>
