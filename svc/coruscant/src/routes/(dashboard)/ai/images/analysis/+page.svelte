<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import { Button } from "@lerpz/ui/components/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import {
    createInfiniteQuery,
    type InfiniteData,
    useQueryClient,
} from "@tanstack/svelte-query";
import {
    analyzeImage,
    getListImagesUrl,
    listImages,
} from "$lib/api/images/images.js";
import type {
    ImageAnalysisResponse,
    ImageItem,
    ImageListResponse,
} from "$lib/api/models/index.js";
import { ErrorDialog } from "$lib/components/error-dialog";
import { toProblemError } from "$lib/components/error-dialog/problem.js";

const PAGE_SIZE = 24;

// The picker is fed by the same list endpoint as the gallery, so images can be
// analysed straight after they're generated.
const query = createInfiniteQuery(() => ({
    queryKey: [getListImagesUrl(), "analysis"],
    queryFn: async ({
        pageParam,
        signal,
    }: {
        pageParam: string | null;
        signal: AbortSignal;
    }): Promise<ImageListResponse> => {
        const res = await listImages(
            {
                limit: PAGE_SIZE,
                ...(pageParam ? { cursor: pageParam } : {}),
            },
            { signal },
        );
        if (res.status !== 200) {
            throw new Error("Failed to load images");
        }
        return res.data;
    },
    initialPageParam: null as string | null,
    getNextPageParam: (lastPage: ImageListResponse) =>
        lastPage.next_cursor ?? undefined,
}));

const images = $derived(query.data?.pages.flatMap((page) => page.items) ?? []);

const queryClient = useQueryClient();

const LIST_QUERY_KEY = [getListImagesUrl(), "analysis"] as const;

let selectedId = $state<string | null>(null);
const selected = $derived(
    images.find((image) => image.id === selectedId) ?? null,
);

let isAnalyzing = $state(false);
let result = $state<ImageAnalysisResponse | null>(null);
// The raw thrown value (a `ProblemSchema`, `Error`, or string) kept so the
// error dialog can render it richly.
let errorValue = $state<unknown>(null);
let errorDialogOpen = $state(false);

// Fall back to any title/tags already persisted on the image so a previously
// analysed image shows its metadata before it's re-run.
const analysis = $derived<ImageAnalysisResponse | null>(
    result ??
        (selected && (selected.title || selected.tags.length > 0)
            ? { title: selected.title ?? "", tags: selected.tags }
            : null),
);

function selectImage(image: ImageItem) {
    if (image.id === selectedId) return;
    selectedId = image.id;
    result = null;
}

async function analyze() {
    if (!selected || isAnalyzing) return;
    isAnalyzing = true;
    try {
        const res = await analyzeImage(selected.id);
        if (res.status !== 200) {
            throw new Error(`Failed to analyse image (${res.status})`);
        }
        result = res.data;

        // Persist the new title/tags into the cached list so re-selecting the
        // image (and the gallery) reflects the analysis without a refetch.
        const analyzedId = selected.id;
        const { title, tags } = res.data;
        queryClient.setQueryData<
            InfiniteData<ImageListResponse, string | null>
        >(LIST_QUERY_KEY, (data) => {
            if (!data) return data;
            return {
                ...data,
                pages: data.pages.map((page) => ({
                    ...page,
                    items: page.items.map((item) =>
                        item.id === analyzedId
                            ? { ...item, title, tags }
                            : item,
                    ),
                })),
            };
        });
    } catch (err) {
        errorValue = toProblemError(err);
        errorDialogOpen = true;
    } finally {
        isAnalyzing = false;
    }
}

// Skeleton placeholders for the picker while the first page loads.
const skeletonCount = 9;
</script>

<div class="mx-auto flex w-full max-w-6xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight">Image analysis</h1>
    <p class="text-sm text-muted-foreground">
      Pick a generated image to detect a title and tags with a vision model.
    </p>
  </header>

  <div class="grid gap-6 lg:grid-cols-2">
    <!-- Picker -->
    <Card class="flex flex-col">
      <CardHeader>
        <CardTitle>Select an image</CardTitle>
        <CardDescription>
          Choose one of your generated images to analyse.
        </CardDescription>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col gap-4">
        {#if query.isLoading}
          <div class="grid grid-cols-3 gap-2">
            {#each Array(skeletonCount) as _, i (i)}
              <Skeleton class="aspect-square w-full rounded-lg" />
            {/each}
          </div>
        {:else if query.isError}
          <p class="rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
            {query.error instanceof Error
              ? query.error.message
              : "Failed to load images."}
          </p>
        {:else if images.length === 0}
          <div class="flex flex-1 flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border px-4 py-16 text-center">
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
          <div class="grid max-h-112 grid-cols-3 gap-2 overflow-y-auto p-1">
            {#each images as image (image.id)}
              {@const isSelected = image.id === selectedId}
              <button
                type="button"
                onclick={() => selectImage(image)}
                aria-pressed={isSelected}
                title={image.title ?? image.prompt}
                class="group relative aspect-square cursor-pointer overflow-hidden rounded-lg border bg-muted/30 outline-none transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring {isSelected
                  ? 'border-primary ring-2 ring-primary'
                  : 'border-border hover:border-primary/50'}"
              >
                <img
                  src={image.url}
                  alt={image.title ?? image.prompt}
                  loading="lazy"
                  class="size-full object-cover"
                />
                {#if isSelected}
                  <span
                    class="absolute right-1.5 top-1.5 flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground"
                  >
                    <Icon icon="fa6-solid:check" class="size-2.5" />
                  </span>
                {/if}
              </button>
            {/each}
          </div>

          {#if query.hasNextPage}
            <div class="flex justify-center">
              <Button
                variant="outline"
                size="sm"
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
      </CardContent>
    </Card>

    <!-- Analysis -->
    <Card class="flex flex-col">
      <CardHeader>
        <CardTitle>Analysis</CardTitle>
        <CardDescription>
          The generated title and tags for the selected image.
        </CardDescription>
      </CardHeader>
      <CardContent class="flex flex-1 flex-col gap-6">
        {#if !selected}
          <div class="flex flex-1 flex-col items-center justify-center gap-3 text-center text-muted-foreground">
            <Icon
              icon="fa6-solid:magnifying-glass-chart"
              class="size-8 text-muted-foreground/60"
            />
            <div class="flex flex-col gap-1">
              <p class="text-base font-medium">No image selected</p>
              <p class="text-sm">
                Pick an image on the left, then run the analysis.
              </p>
            </div>
          </div>
        {:else}
          <div class="overflow-hidden rounded-xl border border-border bg-muted/30">
            <img
              src={selected.url}
              alt={selected.title ?? selected.prompt}
              class="max-h-72 w-full object-contain"
            />
          </div>

          {#if isAnalyzing}
            <div class="flex flex-col gap-3">
              <Skeleton class="h-5 w-2/3 rounded" />
              <div class="flex flex-wrap gap-2">
                {#each Array(5) as _, i (i)}
                  <Skeleton class="h-6 w-16 rounded-full" />
                {/each}
              </div>
            </div>
          {:else if analysis}
            <div class="flex flex-col gap-2">
              <h3 class="text-sm font-medium text-muted-foreground">Title</h3>
              <p class="text-base font-medium">
                {analysis.title || "Untitled"}
              </p>
            </div>
            <div class="flex flex-col gap-2">
              <h3 class="text-sm font-medium text-muted-foreground">Tags</h3>
              {#if analysis.tags.length > 0}
                <div class="flex flex-wrap gap-2">
                  {#each analysis.tags as tag (tag)}
                    <Badge variant="outline">{tag}</Badge>
                  {/each}
                </div>
              {:else}
                <p class="text-sm text-muted-foreground">No tags returned.</p>
              {/if}
            </div>
          {:else}
            <p class="text-sm text-muted-foreground">
              This image hasn't been analysed yet.
            </p>
          {/if}

          <div class="mt-auto">
            <Button onclick={analyze} disabled={isAnalyzing} class="w-full">
              {#if isAnalyzing}
                <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
                Analyzing…
              {:else}
                <Icon icon="fa6-solid:wand-magic-sparkles" class="size-4" />
                {analysis ? "Re-analyze" : "Analyze"}
              {/if}
            </Button>
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>

<ErrorDialog bind:open={errorDialogOpen} error={errorValue} />
