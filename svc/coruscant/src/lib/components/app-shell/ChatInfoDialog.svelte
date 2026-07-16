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
import { cn } from "@lerpz/ui/lib/utils";
import { toast } from "svelte-sonner";
import type { Conversation } from "$lib/api/models/index.js";

let {
    open = $bindable(false),
    conversation,
    onOpenChange,
}: {
    /** Controls visibility of the dialog. Bindable. */
    open?: boolean;
    /** The conversation whose details are displayed. */
    conversation?: Conversation | null;
    /** Called whenever the open state changes (backdrop click, Escape, …). */
    onOpenChange?: (open: boolean) => void;
} = $props();

const title = $derived(conversation?.title ?? "Untitled");

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

let copied = $state(false);
let copyTimeout: ReturnType<typeof setTimeout> | undefined;

async function copyId() {
    const id = conversation?.id;
    if (!id) return;
    try {
        if (!navigator.clipboard) {
            throw new Error(
                "Clipboard access requires a secure (HTTPS) context.",
            );
        }
        await navigator.clipboard.writeText(id);
        copied = true;
        clearTimeout(copyTimeout);
        copyTimeout = setTimeout(() => {
            copied = false;
        }, 1500);
    } catch (err) {
        const reason = err instanceof Error ? err.message : "Unknown error.";
        toast.error("Couldn't copy ID", { description: reason });
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
    <DialogContent class="max-h-[85vh] overflow-hidden">
      <div class="flex flex-col overflow-y-auto">
        <!-- Header -->
        <div class="flex items-start gap-3 p-6 pb-4">
          <span
            class="mt-0.5 flex size-9 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
          >
            <Icon icon="fa6-solid:circle-info" class="size-4.5" />
          </span>
          <div class="min-w-0 flex-1 space-y-1">
            <DialogTitle class="truncate" {title}>{title}</DialogTitle>
            <DialogDescription>Conversation details</DialogDescription>
          </div>
          <DialogClose
            aria-label="Close"
            class="-mr-2 -mt-2 flex size-8 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
          >
            <Icon icon="fa6-solid:xmark" class="size-4" />
          </DialogClose>
        </div>

        <!-- Details -->
        <dl class="space-y-3 px-6 pb-2 text-sm">
          <div class="flex items-center justify-between gap-4">
            <dt class="text-muted-foreground">Model</dt>
            <dd class="min-w-0">
              <span class="inline-flex max-w-full items-center truncate rounded-md bg-muted px-2 py-0.5 text-xs">
                {conversation?.model ?? "—"}
              </span>
            </dd>
          </div>
          <div class="flex items-center justify-between gap-4">
            <dt class="text-muted-foreground">Status</dt>
            <dd class="font-medium">
              {conversation?.archived ? "Archived" : "Active"}
            </dd>
          </div>
          <div class="flex items-center justify-between gap-4">
            <dt class="text-muted-foreground">Created</dt>
            <dd class="font-medium">{formatDate(conversation?.created_at)}</dd>
          </div>
          <div class="flex items-center justify-between gap-4">
            <dt class="text-muted-foreground">Last updated</dt>
            <dd class="font-medium">{formatDate(conversation?.updated_at)}</dd>
          </div>
        </dl>

        <!-- Conversation ID -->
        <div class="px-6 pb-2 pt-2">
          <p class="mb-1.5 px-0.5 text-xs font-medium text-muted-foreground">
            Conversation ID
          </p>
          <div
            class="flex items-center gap-2 rounded-lg border bg-muted/40 px-3 py-2"
          >
            <p
              class="min-w-0 flex-1 truncate font-mono text-sm"
              title={conversation?.id}
            >
              {conversation?.id ?? "—"}
            </p>
            <Button
              variant="ghost"
              size="xs"
              onclick={copyId}
              aria-label={copied ? "Copied ID" : "Copy ID"}
              class="shrink-0 text-muted-foreground hover:bg-background hover:text-foreground"
            >
              <Icon
                icon={copied ? "fa6-solid:check" : "fa6-regular:copy"}
                class="size-3.5"
              />
              <span>{copied ? "Copied" : "Copy"}</span>
            </Button>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center justify-end gap-2 p-6 pt-4">
          <DialogClose
            class={cn(buttonVariants({ variant: "default", size: "sm" }))}
          >
            Close
          </DialogClose>
        </div>
      </div>
    </DialogContent>
  </DialogPositioner>
</Dialog>
