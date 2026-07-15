<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createInfiniteQuery } from "@tanstack/svelte-query";
import { authenticatedFetch } from "$lib/http/fetch.js";

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
    <div class="columns-2 gap-3 sm:columns-3 lg:columns-4 [&>*]:mb-3">
      {#each skeletonHeights as height, i (i)}
        <Skeleton
          class="w-full break-inside-avoid rounded-xl"
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
    <div class="columns-2 gap-3 sm:columns-3 lg:columns-4 [&>*]:mb-3">
      {#each images as image (image.id)}
        <a
          href={image.url}
          target="_blank"
          rel="noopener noreferrer"
          class="group relative block break-inside-avoid overflow-hidden rounded-xl border border-border bg-muted/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
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
        </a>
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
