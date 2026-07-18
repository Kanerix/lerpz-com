<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import {
    createInfiniteQuery,
    type InfiniteData,
    useQueryClient,
} from "@tanstack/svelte-query";
import { toast } from "svelte-sonner";
import type { ImageItem } from "$lib/api/models/index.js";
import { authenticatedFetch } from "$lib/http/fetch.js";
import { downloadImage } from "$lib/utils/download.js";
import { fade, fly } from "$lib/utils/transitions.js";
import ImageDetailDialog from "./ImageDetailDialog.svelte";

// DRAFT: this page fetches the paginated image list directly via
// `authenticatedFetch`. Once the kamino OpenAPI spec is regenerated
// (`bun run generate:api`), the hand-rolled `fetchImages` below can be replaced
// with the generated `listImages` client from `$lib/api/images/images.ts`.

type GalleryImage = {
    id: string;
    url: string;
    prompt: string;
    model: string;
    title: string | null;
    tags: string[];
    format: string;
    width: number;
    height: number;
    created_at: string;
};

type ImageListResponse = {
    items: GalleryImage[];
    next_cursor: string | null;
};

const PAGE_SIZE = 24;

async function fetchImages(
    cursor: string | null,
    signal: AbortSignal,
): Promise<ImageListResponse> {
    const params = new URLSearchParams({ limit: String(PAGE_SIZE) });
    if (cursor) params.set("cursor", cursor);

    const res = await authenticatedFetch(`/api/v1/images?${params}`, {
        signal,
    });
    if (!res.ok) {
        throw new Error(`Failed to load images (${res.status})`);
    }
    return res.json() as Promise<ImageListResponse>;
}

const query = createInfiniteQuery(() => ({
    queryKey: ["/api/v1/images", "gallery"],
    queryFn: ({
        pageParam,
        signal,
    }: {
        pageParam: string | null;
        signal: AbortSignal;
    }) => fetchImages(pageParam, signal),
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: ImageListResponse) => lastPage.next_cursor,
}));

const images = $derived(query.data?.pages.flatMap((page) => page.items) ?? []);

const queryClient = useQueryClient();

const GALLERY_QUERY_KEY = ["/api/v1/images", "gallery"] as const;

// Images currently being deleted, so their card can show a spinner and ignore
// repeat clicks while the request is in flight.
let pendingIds = $state<string[]>([]);

async function handleDelete(image: GalleryImage) {
    if (pendingIds.includes(image.id)) return;
    pendingIds = [...pendingIds, image.id];
    try {
        const res = await authenticatedFetch(`/api/v1/images/${image.id}`, {
            method: "DELETE",
        });
        if (!res.ok) {
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

// The image shown in the detail popover, and whether it's open.
let activeImage = $state<GalleryImage | null>(null);
let detailOpen = $state(false);

function openDetail(image: GalleryImage) {
    activeImage = image;
    detailOpen = true;
}

// A surrounding image picked from inside the popover may not live in the loaded
// gallery pages, so normalise it into a `GalleryImage` before showing it.
function selectDetailImage(image: ImageItem) {
    activeImage = { ...image, title: image.title ?? null };
}

// Persist a fresh analysis into every cached page (and the active image) so the
// gallery and popover reflect the new title/tags without a refetch.
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
// masonry grid rather than a uniform block.
const skeletonHeights = [220, 300, 180, 260, 200, 320, 240, 280];
</script>

<div class="mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight">Gallery</h1>
    <p class="text-sm text-muted-foreground">
      Every image you've generated, newest first.
    </p>
  </header>

  {#if query.isLoading}
    <div class="flex flex-wrap justify-center gap-3">
      {#each skeletonHeights as height, i (i)}
        <Skeleton
          class="w-72 max-w-full rounded-xl"
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
    <div class="flex flex-col items-center gap-3 rounded-xl border border-dashed border-border px-4 py-16 text-center">
      <Icon
        icon="fa6-solid:images"
        class="size-8 text-muted-foreground/60"
      />
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
    <div class="flex flex-wrap justify-center gap-3">
      {#each images as image, i (image.id)}
        {@const isPending = pendingIds.includes(image.id)}
        <div
          class="group relative w-72 max-w-full"
          in:fly|global={{ y: 16, duration: 350, delay: (i % PAGE_SIZE) * 25 }}
          out:fade|global={{ duration: 200 }}
        >
          <button
            type="button"
            onclick={() => openDetail(image)}
            class="block w-full transform-gpu overflow-hidden rounded-xl border border-border bg-muted/30 text-left focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            title={image.title ?? image.prompt}
          >
            <img
              src={image.url}
              alt={image.title ?? image.prompt}
              loading="lazy"
              width={image.width}
              height={image.height}
              style="aspect-ratio: {image.width} / {image.height}"
              class="w-full bg-muted/30 object-cover transition-transform duration-300 group-hover:scale-[1.02]"
            />
            <div
              class="pointer-events-none absolute inset-x-0 bottom-0 translate-y-1 bg-gradient-to-t from-black/70 via-black/30 to-transparent p-3 opacity-0 transition-all duration-200 group-hover:translate-y-0 group-hover:opacity-100"
            >
              <p class="line-clamp-2 text-xs text-white/90">{image.prompt}</p>
              <p class="mt-1 text-[10px] uppercase tracking-wide text-white/60">
                {image.model}
              </p>
            </div>
          </button>

          <DropdownMenu align="end" sideOffset={4}>
            <DropdownMenuTrigger
              aria-label="Image options"
              class="absolute right-2 top-2 flex size-8 items-center justify-center rounded-md bg-black/30 text-white opacity-0 outline-none backdrop-blur-sm transition-opacity hover:bg-black/50 focus-visible:opacity-100 group-hover:opacity-100 data-[state=open]:opacity-100"
            >
              {#if isPending}
                <Icon icon="fa6-solid:spinner" class="size-3.5 animate-spin" />
              {:else}
                <Icon icon="fa6-solid:ellipsis" class="size-3.5" />
              {/if}
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-40">
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

<ImageDetailDialog
  bind:open={detailOpen}
  image={activeImage}
  onSelectImage={selectDetailImage}
  onDownload={handleDownload}
  onAnalyzed={applyAnalysis}
/>
