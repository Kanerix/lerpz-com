<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import {
    createInfiniteQuery,
} from "@tanstack/svelte-query";
import { toast } from "svelte-sonner";
import type { VideoItem, VideoListResponse } from "$lib/api/models/index.js";
import { getListVideosUrl, listVideos } from "$lib/api/videos/videos.js";
import { downloadFile } from "$lib/utils/download.js";
import { formatDuration } from "$lib/utils/format.js";
import { fade, fly } from "$lib/utils/transitions.js";
import VideoDetailDialog from "./VideoDetailDialog.svelte";

const PAGE_SIZE = 24;

const GALLERY_QUERY_KEY = [getListVideosUrl(), "gallery"] as const;

const query = createInfiniteQuery(() => ({
    queryKey: GALLERY_QUERY_KEY,
    queryFn: async ({
        pageParam,
        signal,
    }: {
        pageParam: string | null;
        signal: AbortSignal;
    }): Promise<VideoListResponse> => {
        const res = await listVideos(
            { cursor: pageParam ?? undefined, limit: PAGE_SIZE },
            { signal },
        );
        if (res.status !== 200) {
            throw new Error(`Failed to load videos (${res.status})`);
        }
        return res.data;
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: VideoListResponse) =>
        lastPage.next_cursor ?? undefined,
}));

const videos = $derived(query.data?.pages.flatMap((page) => page.items) ?? []);

// The video shown in the detail dialog, and whether it's open.
let activeVideo = $state<VideoItem | null>(null);
let detailOpen = $state(false);

function openDetail(video: VideoItem) {
    activeVideo = video;
    detailOpen = true;
}

// A neighbour picked from inside the dialog rail is already a `VideoItem`, so it
// can become the active video directly.
function selectDetailVideo(video: VideoItem) {
    activeVideo = video;
}

async function handleDownload(video: VideoItem) {
    const filename = video.format
        ? `${video.id}.${video.format}`
        : `${video.id}.mp4`;
    try {
        await downloadFile(video.url, filename);
    } catch (err) {
        toast.error("Couldn't download video", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    }
}

// Preview each clip on hover; reset to the poster frame on leave.
function hoverPlay(event: Event) {
    const el = event.currentTarget as HTMLVideoElement;
    el.play?.().catch(() => {
        // Autoplay can be interrupted by a quick leave; ignore.
    });
}

function hoverPause(event: Event) {
    const el = event.currentTarget as HTMLVideoElement;
    el.pause();
    el.currentTime = 0;
}

// Skeleton placeholders mixing landscape and portrait ratios so the loading
// state reads like the real grid.
const skeletonHeights = [180, 384, 180, 384, 240, 180, 384, 180];
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8 sm:px-6">
  <header class="flex items-end justify-between gap-4">
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Gallery</h1>
      <p class="text-sm text-muted-foreground">
        Every video you've generated, newest first.
      </p>
    </div>
    {#if videos.length > 0}
      <span
        class="hidden shrink-0 items-center gap-1.5 rounded-full border border-border bg-muted/40 px-3 py-1 text-xs font-medium text-muted-foreground sm:inline-flex"
      >
        <Icon icon="fa6-solid:film" class="size-3" />
        {videos.length}{query.hasNextPage ? "+" : ""} video{videos.length === 1
          ? ""
          : "s"}
      </span>
    {/if}
  </header>

  {#if query.isLoading}
    <div class="flex flex-wrap justify-center gap-4">
      {#each skeletonHeights as height, i (i)}
        <Skeleton
          class="w-80 max-w-full rounded-2xl"
          style="height: {height}px"
        />
      {/each}
    </div>
  {:else if query.isError}
    <p class="rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
      {query.error instanceof Error
        ? query.error.message
        : "Failed to load videos."}
    </p>
  {:else if videos.length === 0}
    <div class="flex flex-col items-center gap-3 rounded-2xl border border-dashed border-border px-4 py-20 text-center">
      <span
        class="flex size-14 items-center justify-center rounded-2xl bg-muted text-muted-foreground/70"
      >
        <Icon icon="fa6-solid:film" class="size-6" />
      </span>
      <div class="flex flex-col gap-1">
        <p class="text-base font-medium">No videos yet</p>
        <p class="text-sm text-muted-foreground">
          Generate a video and it'll show up here.
        </p>
      </div>
      <Button href="/ai/videos" variant="outline" size="sm" class="mt-1">
        <Icon icon="fa6-solid:wand-magic-sparkles" class="size-3.5" />
        Create a video
      </Button>
    </div>
  {:else}
    <!-- Centered flex-wrap grid: rows always fill and centre, whatever the
         item count, so clips never cluster to one side. -->
    <div class="flex flex-wrap justify-center gap-4">
      {#each videos as video, i (video.id)}
        <div
          class="group relative w-80 max-w-full"
          in:fly|global={{ y: 16, duration: 350, delay: (i % PAGE_SIZE) * 25 }}
          out:fade|global={{ duration: 200 }}
        >
          <button
            type="button"
            onclick={() => openDetail(video)}
            class="block w-full transform-gpu overflow-hidden rounded-2xl border border-border/60 bg-muted/30 text-left shadow-sm ring-0 transition-all duration-300 group-hover:-translate-y-1 group-hover:border-border group-hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            title={video.title ?? video.prompt}
          >
            <!-- svelte-ignore a11y_media_has_caption -->
            <video
              src={video.url}
              muted
              loop
              playsinline
              preload="metadata"
              width={video.width}
              height={video.height}
              style="aspect-ratio: {video.width} / {video.height}"
              onmouseenter={hoverPlay}
              onmouseleave={hoverPause}
              class="w-full bg-muted/30 object-cover transition-transform duration-500 group-hover:scale-[1.03]"
            ></video>

            <!-- Play affordance, fades out once the clip starts on hover. -->
            <span
              class="pointer-events-none absolute inset-0 flex items-center justify-center opacity-100 transition-opacity duration-300 group-hover:opacity-0"
            >
              <span
                class="flex size-12 items-center justify-center rounded-full bg-black/40 text-white shadow-lg ring-1 ring-white/20 backdrop-blur-sm"
              >
                <Icon icon="fa6-solid:play" class="size-4 translate-x-px" />
              </span>
            </span>

            <!-- Duration pill. -->
            <span
              class="pointer-events-none absolute right-2 top-2 flex items-center gap-1 rounded-full bg-black/50 px-2 py-0.5 text-[10px] font-medium text-white tabular-nums backdrop-blur-sm"
            >
              <Icon icon="fa6-regular:clock" class="size-2.5" />
              {formatDuration(video.duration)}
            </span>

            <!-- Prompt + model, revealed on hover. -->
            <div
              class="pointer-events-none absolute inset-x-0 bottom-0 translate-y-1 bg-linear-to-t from-black/80 via-black/35 to-transparent p-3 pt-8 opacity-0 transition-all duration-300 group-hover:translate-y-0 group-hover:opacity-100"
            >
              <p class="line-clamp-2 text-xs leading-snug text-white/90">
                {video.prompt}
              </p>
              <p class="mt-1 text-[10px] uppercase tracking-wide text-white/60">
                {video.model}
              </p>
            </div>
          </button>

          <DropdownMenu align="end" sideOffset={4}>
            <DropdownMenuTrigger
              aria-label="Video options"
              class="absolute left-2 top-2 flex size-8 items-center justify-center rounded-lg bg-black/35 text-white opacity-0 outline-none backdrop-blur-sm transition-opacity hover:bg-black/55 focus-visible:opacity-100 group-hover:opacity-100 data-[state=open]:opacity-100"
            >
              <Icon icon="fa6-solid:ellipsis" class="size-3.5" />
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40">
              <DropdownMenuItem
                value="open"
                onclick={() => openDetail(video)}
              >
                <Icon icon="fa6-solid:up-right-and-down-left-from-center" />
                Open
              </DropdownMenuItem>
              <DropdownMenuItem
                value="download"
                onclick={() => handleDownload(video)}
              >
                <Icon icon="fa6-solid:download" />
                Download
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      {/each}
    </div>

    {#if query.hasNextPage}
      <div class="flex justify-center">
        <Button
          variant="outline"
          onclick={() => query.fetchNextPage()}
          disabled={query.isFetchingNextPage}
        >
          {#if query.isFetchingNextPage}
            <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
            Loading…
          {:else}
            Load more
          {/if}
        </Button>
      </div>
    {/if}
  {/if}
</div>
</ScrollArea>

<VideoDetailDialog
  bind:open={detailOpen}
  video={activeVideo}
  onSelectVideo={selectDetailVideo}
  onDownload={handleDownload}
/>
