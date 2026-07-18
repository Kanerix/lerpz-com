<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
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
import { toast } from "svelte-sonner";
import {
    analyzeImage,
    getListImagesUrl,
    listImages,
} from "$lib/api/images/images.js";
import type {
    ImageItem,
    ImageListResponse,
} from "$lib/api/models/index.js";
import { fade, fly } from "$lib/utils/transitions.js";

// Number of surrounding images to load for the "more like this" rail. Kept
// small so the popover stays focused on the selected image.
const NEARBY_LIMIT = 5;

let {
    open = $bindable(false),
    image = null,
    onSelectImage,
    onDownload,
    onAnalyzed,
    onOpenChange,
}: {
    /** Controls visibility of the dialog. Bindable. */
    open?: boolean;
    /** The image whose details are displayed. */
    image?: ImageItem | null;
    /** Called when a surrounding image is clicked, to swap the active image. */
    onSelectImage?: (image: ImageItem) => void;
    /** Download the given image. */
    onDownload?: (image: ImageItem) => void;
    /** Called after a successful analysis so the caller can persist the result. */
    onAnalyzed?: (id: string, title: string, tags: string[]) => void;
    /** Called whenever the open state changes (backdrop click, Escape, …). */
    onOpenChange?: (open: boolean) => void;
} = $props();

// Surrounding images: everything older than the current image, using its ID as
// the pagination cursor.
const nearbyQuery = createQuery(() => ({
    queryKey: [getListImagesUrl(), "nearby", image?.id],
    enabled: Boolean(image),
    queryFn: async ({
        signal,
    }: {
        signal: AbortSignal;
    }): Promise<ImageListResponse> => {
        const res = await listImages(
            { cursor: image?.id, limit: NEARBY_LIMIT },
            { signal },
        );
        if (res.status !== 200) {
            throw new Error("Failed to load images");
        }
        return res.data;
    },
}));

const nearbyImages = $derived(nearbyQuery.data?.items ?? []);

let isAnalyzing = $state(false);
// Locally analysed title/tags, cleared whenever the active image changes so we
// never show one image's analysis on another.
let result = $state<{ title: string; tags: string[] } | null>(null);
let lastAnalyzedId = $state<string | null>(null);

$effect(() => {
    if (image?.id !== lastAnalyzedId) {
        result = null;
    }
});

// Fall back to any title/tags already persisted on the image so a previously
// analysed image shows its metadata before it's re-run.
const analysis = $derived<{ title: string; tags: string[] } | null>(
    result ??
        (image && (image.title || image.tags.length > 0)
            ? { title: image.title ?? "", tags: image.tags }
            : null),
);

// Simple label/value rows, derived so they can be rendered (and animated) in a
// single staggered loop.
const detailRows = $derived(
    image
        ? [
              { label: "Model", value: image.model, badge: true },
              {
                  label: "Dimensions",
                  value: `${image.width} × ${image.height}`,
                  badge: false,
              },
              {
                  label: "Format",
                  value: image.format.toUpperCase(),
                  badge: false,
              },
              {
                  label: "Created",
                  value: formatDate(image.created_at),
                  badge: false,
              },
          ]
        : [],
);

function formatDate(value: string | null | undefined): string {
    if (!value) return "—";
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return "—";
    return date.toLocaleString(undefined, {
        year: "numeric",
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
    });
}

async function analyze() {
    if (!image || isAnalyzing) return;
    const target = image;
    isAnalyzing = true;
    try {
        const res = await analyzeImage(target.id);
        if (res.status !== 200) {
            throw new Error(`Failed to analyse image (${res.status})`);
        }
        lastAnalyzedId = target.id;
        result = res.data;
        onAnalyzed?.(target.id, res.data.title, res.data.tags);
    } catch (err) {
        toast.error("Couldn't analyse image", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    } finally {
        isAnalyzing = false;
    }
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
      {#if image}
        <div class="flex max-h-[88vh] flex-col overflow-hidden md:flex-row">
          <!-- Image -->
          <div
            class="flex min-h-64 flex-1 items-center justify-center overflow-hidden bg-muted/40 p-4"
          >
            {#key image.id}
              <img
                src={image.url}
                alt={image.title ?? image.prompt}
                width={image.width}
                height={image.height}
                class="max-h-[80vh] w-auto max-w-full rounded-lg object-contain shadow-lg"
                in:fly|global={{ y: 12, duration: 400 }}
              />
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
                {#key analysis?.title || image.title}
                  <div in:fade|global={{ duration: 250 }}>
                    <DialogTitle class="truncate text-base">
                      {analysis?.title || image.title || "Untitled"}
                    </DialogTitle>
                  </div>
                {/key}
                <p class="text-xs text-muted-foreground">Image details</p>
              </div>
              <DialogClose
                aria-label="Close"
                class="-mr-1 -mt-1 flex size-8 shrink-0 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
              >
                <Icon icon="fa6-solid:xmark" class="size-4" />
              </DialogClose>
            </div>

            {#key image.id}
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
                    {image.prompt}
                  </p>
                </ScrollArea>
              </div>

              <!-- Metadata takes exactly the space it needs. -->
              <dl class="shrink-0 space-y-4 px-5 pt-4 pb-5 text-sm">
                {#each detailRows as row, i (row.label)}
                  <div
                    class="flex items-center justify-between gap-4"
                    in:fly|global={{
                      y: 8,
                      duration: 300,
                      delay: 80 + i * 40,
                    }}
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

                <div
                  class="flex flex-col gap-1.5"
                  in:fly|global={{ y: 8, duration: 300, delay: 240 }}
                >
                  <dt class="text-xs font-medium text-muted-foreground">
                    Tags
                  </dt>
                  <dd>
                    {#if isAnalyzing}
                      <div class="flex flex-wrap gap-1.5">
                        {#each Array(4) as _, i (i)}
                          <Skeleton class="h-5 w-14 rounded-full" />
                        {/each}
                      </div>
                    {:else if analysis && analysis.tags.length > 0}
                      <div class="flex flex-wrap gap-1.5">
                        {#each analysis.tags as tag, i (tag)}
                          <span
                            in:fly|global={{
                              y: 6,
                              duration: 250,
                              delay: i * 40,
                            }}
                          >
                            <Badge variant="outline">{tag}</Badge>
                          </span>
                        {/each}
                      </div>
                    {:else}
                      <span class="text-sm text-muted-foreground">
                        No tags yet.
                      </span>
                    {/if}
                  </dd>
                </div>
              </dl>
            {/key}

            <div
              class="flex flex-col gap-2 border-t p-5"
              in:fly|global={{ y: 8, duration: 300, delay: 120 }}
            >
              <Button onclick={analyze} disabled={isAnalyzing}>
                {#if isAnalyzing}
                  <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
                  Analyzing…
                {:else}
                  <Icon icon="fa6-solid:wand-magic-sparkles" class="size-4" />
                  {analysis ? "Re-analyze" : "Analyze"}
                {/if}
              </Button>
              {#if onDownload}
                <Button variant="outline" onclick={() => onDownload?.(image)}>
                  <Icon icon="fa6-solid:download" class="size-4" />
                  Download
                </Button>
              {/if}
            </div>
          </div>

          <!-- Surrounding images -->
          <div
            class="flex w-full flex-col overflow-hidden border-t md:w-48 md:shrink-0 md:border-t-0 md:border-l"
          >
            <p class="px-4 pt-4 pb-2 text-xs font-medium text-muted-foreground">
              More images
            </p>
            <ScrollArea orientation="vertical" class="flex-1">
              <div class="flex flex-col gap-2 px-4 pb-4">
                {#if nearbyQuery.isLoading}
                  {#each Array(NEARBY_LIMIT) as _, i (i)}
                    <Skeleton class="aspect-square w-full rounded-lg" />
                  {/each}
                {:else if nearbyQuery.isError}
                  <p class="text-xs text-destructive">
                    Failed to load more images.
                  </p>
                {:else if nearbyImages.length === 0}
                  <p class="text-xs text-muted-foreground">
                    No more images to show.
                  </p>
                {:else}
                  {#each nearbyImages as item, i (item.id)}
                    <button
                      type="button"
                      onclick={() => onSelectImage?.(item)}
                      title={item.title ?? item.prompt}
                      class="group relative aspect-square overflow-hidden rounded-lg border border-border bg-muted/30 outline-none transition hover:border-primary/50 hover:shadow-md focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
                      in:fly|global={{ y: 12, duration: 300, delay: i * 50 }}
                    >
                      <img
                        src={item.url}
                        alt={item.title ?? item.prompt}
                        loading="lazy"
                        class="size-full object-cover transition-transform duration-300 group-hover:scale-[1.05]"
                      />
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
