<script lang="ts">
import Icon from "@iconify/svelte";
import { Button, buttonVariants } from "@lerpz/ui/components/button";
import {
    Dialog,
    DialogBackdrop,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogPositioner,
    DialogTitle,
} from "@lerpz/ui/components/dialog";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { cn } from "@lerpz/ui/lib/utils";
import { toast } from "svelte-sonner";
import { highlightJson, stringifyJson } from "./json-highlight.js";
import {
    getErrorMessage,
    getErrorTraceback,
    isProblemSchema,
} from "./problem.js";

let {
    open = $bindable(false),
    error,
    onOpenChange,
}: {
    /** Controls visibility of the dialog. Bindable. */
    open?: boolean;
    /**
     * The value to display. An RFC 9457 problem is the expected shape and is
     * rendered as a rich summary; anything else falls back to a generic error
     * view with a full traceback.
     */
    error?: unknown;
    /** Called whenever the open state changes (e.g. backdrop click, Escape). */
    onOpenChange?: (open: boolean) => void;
} = $props();

// Default path: a well-formed problem+json response.
const problem = $derived(isProblemSchema(error) ? error : null);
// Fallback path: any other non-empty thrown value.
const isFallback = $derived(!problem && error !== undefined && error !== null);

// A `type` other than `about:blank` dereferences to human-readable
// documentation for the problem (per RFC 9457), so that's the only case where
// the docs button is meaningful.
const docsUrl = $derived.by(() => {
    const type = problem?.type;
    if (!type || type === "about:blank") return null;
    try {
        const url = new URL(type);
        if (url.protocol !== "http:" && url.protocol !== "https:") return null;
        return url.toString();
    } catch {
        return null;
    }
});

const logId = $derived(problem?.log_id ?? null);

// Highlight the *entire* response, including the `extension` field, which is
// intentionally hidden from the summary above and only surfaced here.
const highlightedJson = $derived(problem ? highlightJson(problem) : "");
const jsonText = $derived(problem ? stringifyJson(problem) : "");

const fallbackMessage = $derived(isFallback ? getErrorMessage(error) : "");
const traceback = $derived(isFallback ? getErrorTraceback(error) : "");

let showDetails = $state(false);
// Tracks which item was most recently copied so only that button shows the
// "Copied" state (the Log ID and response copy buttons are visible at once).
let copiedKey = $state<string | null>(null);
let copyTimeout: ReturnType<typeof setTimeout> | undefined;

async function copyToClipboard(text: string, key: string, label: string) {
    if (!text) return;
    try {
        if (!navigator.clipboard) {
            throw new Error(
                "Clipboard access requires a secure (HTTPS) context.",
            );
        }
        await navigator.clipboard.writeText(text);
        copiedKey = key;
        clearTimeout(copyTimeout);
        copyTimeout = setTimeout(() => {
            copiedKey = null;
        }, 1500);
    } catch (err) {
        const reason = err instanceof Error ? err.message : "Unknown error.";
        toast.error(`Couldn't copy ${label}`, { description: reason });
    }
}

function handleOpenChange(details: { open: boolean }) {
    open = details.open;
    if (!details.open) showDetails = false;
    onOpenChange?.(details.open);
}
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogBackdrop />
  <DialogPositioner>
    <DialogContent class="max-h-[85vh] overflow-hidden">
      {#if problem}
        <div class="flex flex-col overflow-y-auto">
          <!-- Header: title + description -->
          <div class="flex items-start gap-3 p-6 pb-4">
            <span
              class="mt-0.5 flex size-9 shrink-0 items-center justify-center rounded-full bg-destructive/10 text-destructive"
            >
              <Icon icon="fa6-solid:circle-exclamation" class="size-4.5" />
            </span>
            <div class="min-w-0 flex-1 space-y-1">
              <DialogTitle>{problem.title}</DialogTitle>
              <DialogDescription>{problem.detail}</DialogDescription>
            </div>
            <DialogClose
              aria-label="Close"
              class="-mr-2 -mt-2 flex size-8 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            >
              <Icon icon="fa6-solid:xmark" class="size-4" />
            </DialogClose>
          </div>

          <!-- Log ID -->
          {#if logId}
            <div class="px-6 pb-2">
              <p class="mb-1.5 px-0.5 text-xs font-medium text-muted-foreground">
                Log ID
              </p>
              <div
                class="flex items-center gap-2 rounded-lg border bg-muted/40 px-3 py-2"
              >
                <p
                  class="min-w-0 flex-1 truncate font-mono text-sm"
                  title={logId}
                >
                  {logId}
                </p>
                <button
                  type="button"
                  onclick={() => copyToClipboard(logId, "logId", "Log ID")}
                  aria-label={copiedKey === "logId"
                    ? "Copied Log ID"
                    : "Copy Log ID"}
                  class="inline-flex shrink-0 items-center gap-1.5 rounded-md px-2 py-1 text-xs text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
                >
                  <Icon
                    icon={copiedKey === "logId"
                      ? "fa6-solid:check"
                      : "fa6-regular:copy"}
                    class="size-3.5"
                  />
                  <span>{copiedKey === "logId" ? "Copied" : "Copy"}</span>
                </button>
              </div>
              <p class="mt-1.5 px-0.5 text-xs text-muted-foreground">
                Share this Log ID with the developers so they can trace the
                error.
              </p>
            </div>
          {/if}

          <!-- Full JSON preview -->
          {#if showDetails}
            <div class="json-preview px-6 pt-2">
              <div class="overflow-hidden rounded-lg border bg-muted/40">
                <div
                  class="flex items-center justify-between border-b px-3 py-1.5"
                >
                  <span
                    class="font-mono text-xs font-medium text-muted-foreground"
                  >
                    Response
                  </span>
                  <button
                    type="button"
                    onclick={() =>
                      copyToClipboard(jsonText, "response", "response")}
                    aria-label={copiedKey === "response"
                      ? "Copied response"
                      : "Copy response"}
                    class="inline-flex shrink-0 items-center gap-1.5 rounded-md px-2 py-0.5 text-xs text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
                  >
                    <Icon
                      icon={copiedKey === "response"
                        ? "fa6-solid:check"
                        : "fa6-regular:copy"}
                      class="size-3.5"
                    />
                    <span>{copiedKey === "response" ? "Copied" : "Copy"}</span>
                  </button>
                </div>
                <ScrollArea orientation="both" class="max-h-64">
                  <pre
                    class="p-3 text-xs leading-relaxed"><code
                      class="hljs">{@html highlightedJson}</code></pre>
                </ScrollArea>
              </div>
            </div>
          {/if}

          <!-- Actions -->
          <div class="flex flex-wrap items-center gap-2 p-6 pt-4">
            <Button
              variant="ghost"
              size="sm"
              onclick={() => (showDetails = !showDetails)}
            >
              <Icon
                icon={showDetails ? "fa6-solid:chevron-up" : "fa6-solid:code"}
                class="size-3.5"
              />
              {showDetails ? "Hide response" : "Show full response"}
            </Button>

            {#if docsUrl}
              <Button
                variant="outline"
                size="sm"
                href={docsUrl}
                target="_blank"
                rel="noreferrer noopener"
                class="ml-auto"
              >
                <Icon icon="fa6-solid:book" class="size-3.5" />
                View documentation
                <Icon
                  icon="fa6-solid:arrow-up-right-from-square"
                  class="size-3"
                />
              </Button>
            {/if}

            <DialogClose
              class={cn(
                buttonVariants({ variant: "default", size: "sm" }),
                docsUrl ? "" : "ml-auto",
              )}
            >
              Dismiss
            </DialogClose>
          </div>
        </div>
      {:else if isFallback}
        <div class="flex flex-col overflow-y-auto">
          <!-- Header: generic error summary -->
          <div class="flex items-start gap-3 p-6 pb-4">
            <span
              class="mt-0.5 flex size-9 shrink-0 items-center justify-center rounded-full bg-destructive/10 text-destructive"
            >
              <Icon icon="fa6-solid:triangle-exclamation" class="size-4.5" />
            </span>
            <div class="min-w-0 flex-1 space-y-1">
              <DialogTitle>Something went wrong</DialogTitle>
              <DialogDescription>{fallbackMessage}</DialogDescription>
            </div>
            <DialogClose
              aria-label="Close"
              class="-mr-2 -mt-2 flex size-8 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            >
              <Icon icon="fa6-solid:xmark" class="size-4" />
            </DialogClose>
          </div>

          <!-- Full traceback -->
          {#if showDetails}
            <div class="px-6 pt-2">
              <div
                class="overflow-hidden rounded-lg border bg-muted/40"
              >
                <div
                  class="flex items-center justify-between border-b px-3 py-1.5"
                >
                  <span
                    class="font-mono text-xs font-medium text-muted-foreground"
                  >
                    Traceback
                  </span>
                  <button
                    type="button"
                    onclick={() =>
                      copyToClipboard(traceback, "traceback", "traceback")}
                    aria-label={copiedKey === "traceback"
                      ? "Copied traceback"
                      : "Copy traceback"}
                    class="inline-flex shrink-0 items-center gap-1.5 rounded-md px-2 py-0.5 text-xs text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
                  >
                    <Icon
                      icon={copiedKey === "traceback"
                        ? "fa6-solid:check"
                        : "fa6-regular:copy"}
                      class="size-3.5"
                    />
                    <span>{copiedKey === "traceback" ? "Copied" : "Copy"}</span>
                  </button>
                </div>
                <pre
                  class="max-h-64 overflow-auto p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap break-words text-foreground/80">{traceback}</pre>
              </div>
            </div>
          {/if}

          <!-- Actions -->
          <div class="flex flex-wrap items-center gap-2 p-6 pt-4">
            <Button
              variant="ghost"
              size="sm"
              onclick={() => (showDetails = !showDetails)}
            >
              <Icon
                icon={showDetails
                  ? "fa6-solid:chevron-up"
                  : "fa6-solid:terminal"}
                class="size-3.5"
              />
              {showDetails ? "Hide traceback" : "Show traceback"}
            </Button>

            <DialogClose
              class={cn(
                buttonVariants({ variant: "default", size: "sm" }),
                "ml-auto",
              )}
            >
              Dismiss
            </DialogClose>
          </div>
        </div>
      {/if}
    </DialogContent>
  </DialogPositioner>
</Dialog>

<style>
/* highlight.js token theme (GitHub-style), matched to class-based dark mode. */
.json-preview :global(.hljs) {
    color: var(--foreground);
    background: transparent;
}
.json-preview :global(.hljs-attr) {
    color: #005cc5;
}
.json-preview :global(.hljs-string) {
    color: #032f62;
}
.json-preview :global(.hljs-number),
.json-preview :global(.hljs-literal) {
    color: #005cc5;
}
.json-preview :global(.hljs-punctuation) {
    color: var(--muted-foreground);
}

:global(.dark) .json-preview :global(.hljs-attr) {
    color: #79c0ff;
}
:global(.dark) .json-preview :global(.hljs-string) {
    color: #a5d6ff;
}
:global(.dark) .json-preview :global(.hljs-number),
:global(.dark) .json-preview :global(.hljs-literal) {
    color: #79c0ff;
}
</style>
