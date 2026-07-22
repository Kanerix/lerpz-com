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
    type InfiniteData,
    useQueryClient,
} from "@tanstack/svelte-query";
import { toast } from "svelte-sonner";
import {
    deleteImage,
    getListImagesUrl,
    listImages,
} from "$lib/api/images/images.js";
import type { ImageItem, ImageListResponse } from "$lib/api/models/index.js";
import { downloadImage } from "$lib/utils/download.js";
import { fade, fly } from "$lib/utils/transitions.js";
import ImageDetailDialog from "./ImageDetailDialog.svelte";

const PAGE_SIZE = 24;

const GALLERY_QUERY_KEY = [getListImagesUrl(), "gallery"] as const;

const query = createInfiniteQuery(() => ({
    queryKey: GALLERY_QUERY_KEY,
    queryFn: async ({
        pageParam,
        signal,
    }: {
        pageParam: string | null;
        signal: AbortSignal;
    }): Promise<ImageListResponse> => {
        const res = await listImages(
            { cursor: pageParam ?? undefined, limit: PAGE_SIZE },
            { signal },
        );
        if (res.status !== 200) {
            throw new Error(`Failed to load images (${res.status})`);
        }
        return res.data;
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: ImageListResponse) =>
        lastPage.next_cursor ?? undefined,
}));

const images = $derived(query.data?.pages.flatMap((page) => page.items) ?? []);

const queryClient = useQueryClient();

// The image shown in the detail dialog, and whether it's open.
let activeImage = $state<ImageItem | null>(null);
let detailOpen = $state(false);

// Images currently being deleted, so their card can show a spinner and ignore
// repeat clicks while the request is in flight.
let pendingIds = $state<string[]>([]);

function openDetail(image: ImageItem) {
    activeImage = image;
    detailOpen = true;
}

// A neighbour picked from inside the dialog rail is already an `ImageItem`, so
// it can become the active image directly.
function selectDetailImage(image: ImageItem) {
    activeImage = image;
}

async function handleDelete(image: ImageItem) {
    if (pendingIds.includes(image.id)) return;
    pendingIds = [...pendingIds, image.id];
    try {
        const res = await deleteImage(image.id);
        if (res.status !== 200) {
            throw new Error(`Failed to delete image (${res.status})`);
        }
        // Drop the image from every cached page so it disappears without
        // refetching (which would reset the infinite pagination).
        queryClient.setQueryData<
            InfiniteData<ImageListResponse, string | null>
        >(GALLERY_QUERY_KEY, (data) => {
            if (!data) return data;
            return {
                ...data,
                pages: data.pages.map((page) => ({
                    ...page,
                    items: page.items.filter((item) => item.id !== image.id),
                })),
            };
        });
        toast.success("Image deleted");
    } catch (err) {
        toast.error("Couldn't delete image", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    } finally {
        pendingIds = pendingIds.filter((id) => id !== image.id);
    }
}

// Persist a fresh analysis into every cached page (and the active image) so the
// gallery and dialog reflect the new title/tags without a refetch.
function applyAnalysis(id: string, title: string, tags: string[]) {
    queryClient.setQueryData<InfiniteData<ImageListResponse, string | null>>(
        GALLERY_QUERY_KEY,
        (data) => {
            if (!data) return data;
            return {
                ...data,
                pages: data.pages.map((page) => ({
                    ...page,
                    items: page.items.map((item) =>
                        item.id === id ? { ...item, title, tags } : item,
                    ),
                })),
            };
        },
    );
    if (activeImage?.id === id) {
        activeImage = { ...activeImage, title, tags };
    }
}

async function handleDownload(image: ImageItem) {
    const filename = image.format ? `${image.id}.${image.format}` : image.id;
    try {
        await downloadImage(image.url, filename);
    } catch (err) {
        toast.error("Couldn't download image", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    }
}

// Skeleton placeholders with varied heights so the loading state reads as a
// mixed-ratio grid rather than a uniform block.
const skeletonHeights = [220, 300, 180, 260, 200, 320, 240, 280];
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8 sm:px-6">
  <header class="flex items-end justify-between gap-4">
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Gallery</h1>
      <p class="text-sm text-muted-foreground">
        Every image you've generated, newest first.
      </p>
    </div>
    {#if images.length > 0}
      <span
        class="hidden shrink-0 items-center gap-1.5 rounded-full border border-border bg-muted/40 px-3 py-1 text-xs font-medium text-muted-foreground sm:inline-flex"
      >
        <Icon icon="fa6-solid:images" class="size-3" />
        {images.length}{query.hasNextPage ? "+" : ""} image{images.length === 1
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
        : "Failed to load images."}
    </p>
  {:else if images.length === 0}
    <div class="flex flex-col items-center gap-3 rounded-2xl border border-dashed border-border px-4 py-20 text-center">
      <span
        class="flex size-14 items-center justify-center rounded-2xl bg-muted text-muted-foreground/70"
      >
        <Icon icon="fa6-solid:images" class="size-6" />
      </span>
      <div class="flex flex-col gap-1">
        <p class="text-base font-medium">No images yet</p>
        <p class="text-sm text-muted-foreground">
          Generate an image and it'll show up here.
        </p>
      </div>
      <Button href="/ai/images" variant="outline" size="sm" class="mt-1">
        <Icon icon="fa6-solid:wand-magic-sparkles" class="size-3.5" />
        Create an image
      </Button>
    </div>
  {:else}
    <!-- Centered flex-wrap grid: rows always fill and centre, whatever the
         item count, so images never cluster to one side. -->
    <div class="flex flex-wrap justify-center gap-4">
      {#each images as image, i (image.id)}
        {@const isPending = pendingIds.includes(image.id)}
        <div
          class="group relative w-80 max-w-full"
          in:fly|global={{ y: 16, duration: 350, delay: (i % PAGE_SIZE) * 25 }}
          out:fade|global={{ duration: 200 }}
        >
          <button
            type="button"
            onclick={() => openDetail(image)}
            class="block w-full transform-gpu overflow-hidden rounded-2xl border border-border/60 bg-muted/30 text-left shadow-sm ring-0 transition-all duration-300 group-hover:-translate-y-1 group-hover:border-border group-hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            title={image.title ?? image.prompt}
          >
            <img
              src={image.url}
              alt={image.title ?? image.prompt}
              loading="lazy"
              width={image.width}
              height={image.height}
              style="aspect-ratio: {image.width} / {image.height}"
              class="w-full bg-muted/30 object-cover transition-transform duration-500 group-hover:scale-[1.03]"
            />
            <div
              class="pointer-events-none absolute inset-x-0 bottom-0 translate-y-1 bg-linear-to-t from-black/80 via-black/35 to-transparent p-3 pt-8 opacity-0 transition-all duration-300 group-hover:translate-y-0 group-hover:opacity-100"
            >
              <p class="line-clamp-2 text-xs leading-snug text-white/90">
                {image.prompt}
              </p>
              <p class="mt-1 text-[10px] uppercase tracking-wide text-white/60">
                {image.model}
              </p>
            </div>
          </button>

          <DropdownMenu align="end" sideOffset={4}>
            <DropdownMenuTrigger
              aria-label="Image options"
              class="absolute left-2 top-2 flex size-8 items-center justify-center rounded-lg bg-black/35 text-white opacity-0 outline-none backdrop-blur-sm transition-opacity hover:bg-black/55 focus-visible:opacity-100 group-hover:opacity-100 data-[state=open]:opacity-100"
            >
              {#if isPending}
                <Icon icon="fa6-solid:spinner" class="size-3.5 animate-spin" />
              {:else}
                <Icon icon="fa6-solid:ellipsis" class="size-3.5" />
              {/if}
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40">
              <DropdownMenuItem value="open" onclick={() => openDetail(image)}>
                <Icon icon="fa6-solid:up-right-and-down-left-from-center" />
                Open
              </DropdownMenuItem>
              <DropdownMenuItem
                value="download"
                onclick={() => handleDownload(image)}
              >
                <Icon icon="fa6-solid:download" />
                Download
              </DropdownMenuItem>
              <DropdownMenuItem
                value="delete"
                disabled={isPending}
                onclick={() => handleDelete(image)}
                class="text-destructive focus:text-destructive hover:text-destructive"
              >
                <Icon icon="fa6-solid:trash" />
                Delete
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

<ImageDetailDialog
  bind:open={detailOpen}
  image={activeImage}
  onSelectImage={selectDetailImage}
  onDownload={handleDownload}
  onAnalyzed={applyAnalysis}
/>
