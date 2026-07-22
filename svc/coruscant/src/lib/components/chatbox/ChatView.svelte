<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Dialog,
    DialogBackdrop,
    DialogContent,
    DialogDescription,
    DialogPositioner,
    DialogTitle,
} from "@lerpz/ui/components/dialog";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Typewriter } from "@lerpz/ui/components/typewriter";
import { cn } from "@lerpz/ui/lib/utils";
import { useQueryClient } from "@tanstack/svelte-query";
import { cubicOut } from "svelte/easing";
import { getAiContext } from "$lib/ai/context.svelte.js";
import {
    deleteChatMessage,
    getChat,
    getGetChatUrl,
    getListChatsUrl,
} from "$lib/api/chats/chats.js";
import type { ConversationMessage } from "$lib/api/models/index.js";
import ModelAvatar from "$lib/components/avatar/ModelAvatar.svelte";
import UserAvatar from "$lib/components/avatar/UserAvatar.svelte";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import CopyButton from "./CopyButton.svelte";
import DeleteButton from "./DeleteButton.svelte";
import EditButton from "./EditButton.svelte";
import Markdown from "./Markdown.svelte";
import ThinkingBlock from "./ThinkingBlock.svelte";

let {
    messages = [],
    isStreaming = false,
    error = null,
    onRetry,
    onDelete,
}: {
    messages: ConversationMessage[];
    isStreaming: boolean;
    error: string | null;
    onRetry?: () => void;
    onDelete?: (id: string) => void;
} = $props();

const ai = getAiContext();
const queryClient = useQueryClient();

let pendingDeleteId = $state<string | null>(null);
let isDeleting = $state(false);

const deleteDialogOpen = $derived(pendingDeleteId !== null);

// Streamed messages are held in local state with client-only `__temp_` ids; the
// server-assigned ids only arrive on a later reload. Actions that hit the API
// (like deletion) need the real id, so once a stream settles we fetch the
// conversation once and remember a temp -> real id mapping. Reconciling ids
// this way keeps the rendered message keys stable, so nothing re-animates.
let serverIds = $state<Record<string, string>>({});
// Plain (non-reactive) guard so the reconcile effect can't fire concurrently
// or re-enter itself while a fetch is in flight.
let reconciling = false;

function resolveServerId(id: string): string {
    return serverIds[id] ?? id;
}

$effect(() => {
    const convId = ai.conversationId;
    if (!convId || isStreaming) return;

    const snapshot = messages;
    const hasUnmapped = snapshot.some(
        (m) => m.id.startsWith("__temp_") && !serverIds[m.id],
    );
    if (!hasUnmapped || reconciling) return;

    reconciling = true;
    getChat(convId)
        .then((resp) => {
            if (resp.status !== 200 || !resp.data) return;
            const remote = resp.data.messages;
            // Only reconcile when the histories line up; a mismatch means the
            // conversation changed mid-flight, so we skip and retry on the next
            // stable state.
            if (remote.length !== snapshot.length) return;
            const next = { ...serverIds };
            for (let i = 0; i < snapshot.length; i++) {
                const local = snapshot[i];
                const server = remote[i];
                if (
                    local?.id.startsWith("__temp_") &&
                    server?.id &&
                    local.role === server.role
                ) {
                    next[local.id] = server.id;
                }
            }
            serverIds = next;
        })
        .catch(() => {
            // Leave ids unmapped; the effect retries when state next changes.
        })
        .finally(() => {
            reconciling = false;
        });
});

const deleteCount = $derived.by(() => {
    if (!pendingDeleteId) return 0;
    const index = messages.findIndex((m) => m.id === pendingDeleteId);
    return index === -1 ? 0 : messages.length - index;
});

function canDeleteMessage(message: ConversationMessage): boolean {
    return (
        !isStreaming &&
        ai.conversationId != null &&
        !resolveServerId(message.id).startsWith("__temp_")
    );
}

function requestDelete(id: string) {
    pendingDeleteId = id;
}

function cancelDelete() {
    if (isDeleting) return;
    pendingDeleteId = null;
}

async function confirmDelete() {
    const convId = ai.conversationId;
    const targetId = pendingDeleteId;
    if (!convId || !targetId) return;

    isDeleting = true;
    try {
        await deleteChatMessage(convId, resolveServerId(targetId));
        // Trim local state immediately, then refresh the cached conversation and
        // chat list so a later reload reflects the server.
        ai.removeChatMessagesFrom(targetId);
        await queryClient.invalidateQueries({
            queryKey: [getGetChatUrl(convId)],
        });
        await queryClient.invalidateQueries({
            queryKey: [getListChatsUrl()],
        });
        onDelete?.(targetId);
        pendingDeleteId = null;
    } catch (err) {
        // Close the confirmation dialog first so the error dialog isn't stacked
        // on top of it.
        pendingDeleteId = null;
        ai.reportChatError(err);
    } finally {
        isDeleting = false;
    }
}

// Fallback family for assistant avatars: the currently selected model. Used for
// the streaming placeholder and any message that predates per-message families.
const selectedFamily = $derived(
    ai.models.find((m) => m.value === chatboxStore.model)?.family ?? null,
);

// When a send fails, the last user message is the one that didn't go through.
// Flag it so we can attach a "Not sent" state and a retry action to it.
const failedMessageId = $derived.by(() => {
    if (!error || isStreaming) return null;
    for (let i = messages.length - 1; i >= 0; i--) {
        const message = messages[i];
        if (message?.role === "user") return message.id;
    }
    return null;
});

// The most recent user message is the only one that can be edited (resent).
const latestUserMessageId = $derived.by(() => {
    for (let i = messages.length - 1; i >= 0; i--) {
        const message = messages[i];
        if (message?.role === "user") return message.id;
    }
    return null;
});

// Distance (px) from the bottom within which we consider the
// user to be "at the bottom" and therefore following the stream.
const BOTTOM_THRESHOLD = 32;

let viewportRef = $state<HTMLDivElement | null>(null);
let contentRef = $state<HTMLDivElement | null>(null);

let autoScroll = $state(true);

const showFollowButton = $derived(!autoScroll && messages.length > 0);

function isAtBottom(el: HTMLElement) {
    return el.scrollHeight - el.scrollTop - el.clientHeight <= BOTTOM_THRESHOLD;
}

function scrollToBottom(smooth = false) {
    if (!viewportRef) return;
    viewportRef.scrollTo({
        top: viewportRef.scrollHeight,
        behavior: smooth ? "smooth" : "auto",
    });
}

function handleScroll() {
    if (!viewportRef) return;
    autoScroll = isAtBottom(viewportRef);
}

function followAgent() {
    autoScroll = true;
    scrollToBottom(false);
}

$effect(() => {
    chatboxStore.setFollowButtonVisible(showFollowButton);
    return () => chatboxStore.setFollowButtonVisible(false);
});

$effect(() => {
    chatboxStore.setFollowAgentHandler(followAgent);
    return () => chatboxStore.setFollowAgentHandler(null);
});

$effect(() => {
    const el = viewportRef;
    if (!el) return;
    el.addEventListener("scroll", handleScroll, { passive: true });
    return () => el.removeEventListener("scroll", handleScroll);
});

$effect(() => {
    if (!contentRef) return;
    const observer = new ResizeObserver(() => {
        if (autoScroll) scrollToBottom();
    });
    observer.observe(contentRef);
    return () => observer.disconnect();
});

let lastUserMessageId = $state<string | null>(null);
$effect(() => {
    const last = messages[messages.length - 1];
    if (last?.role === "user" && last.id !== lastUserMessageId) {
        lastUserMessageId = last.id;
        autoScroll = true;
    }
});

function bubbleIn(_node: Element, { role }: { role: string }) {
    const x = role === "user" ? 16 : -16;
    return {
        duration: 350,
        easing: cubicOut,
        css: (t: number, u: number) =>
            `opacity: ${t};
             transform: translateY(${u * 10}px) translateX(${u * x}px) scale(${0.95 + t * 0.05});`,
    };
}

const EXAMPLE_PROMPTS = [
    {
        icon: "fa6-solid:lightbulb",
        title: "Explain a concept",
        prompt: "Explain how JSON Web Tokens work, and when I should use them.",
    },
    {
        icon: "fa6-solid:code",
        title: "Write some code",
        prompt: "Write a TypeScript function that debounces an async function.",
    },
    {
        icon: "fa6-solid:feather",
        title: "Draft a message",
        prompt: "Draft a friendly release announcement for a new feature in our app.",
    },
    {
        icon: "fa6-solid:list-check",
        title: "Plan a task",
        prompt: "Help me break down migrating a REST API to gRPC into actionable steps.",
    },
];
</script>

{#if messages.length === 0}
  <div class="flex h-full min-h-[60vh] flex-col items-center justify-center px-4 text-center">
    <div class="flex w-full max-w-xl flex-col items-center gap-6">
      <div class="flex flex-col items-center gap-3">
        <ModelAvatar family={selectedFamily} size="lg" />
        <h2 class="text-xl font-semibold tracking-tight">Start a conversation</h2>
        <p class="text-sm text-muted-foreground max-w-sm">
          Type a message below to begin chatting, or pick an example to get started. Your conversation will be saved automatically.
        </p>
      </div>
      <div class="grid w-full gap-2 sm:grid-cols-2">
        {#each EXAMPLE_PROMPTS as example (example.title)}
          <button
            type="button"
            onclick={() => chatboxStore.setPrompt(example.prompt)}
            class="group flex cursor-pointer flex-col gap-1 rounded-xl border border-border bg-card/40 p-3 text-left transition-colors hover:border-primary/40 hover:bg-accent"
          >
            <span class="flex items-center gap-2 text-sm font-medium">
              <Icon icon={example.icon} class="size-3.5 text-muted-foreground transition-colors group-hover:text-primary" />
              {example.title}
            </span>
            <span class="line-clamp-2 text-xs text-muted-foreground">{example.prompt}</span>
          </button>
        {/each}
      </div>
    </div>
  </div>
{:else}
  <div class="relative h-full w-full p-2">
    <ScrollArea bind:viewportRef orientation="vertical" class="h-full w-full">
      <div bind:this={contentRef} class="mx-auto max-w-6xl flex flex-col gap-4 px-3 pt-3 pb-48">
      {#each messages as message, index (message.id)}
        {@const isEditing = chatboxStore.editingMessageId === message.id}
        {@const isFailed = message.id === failedMessageId}
        <div
          in:bubbleIn={{ role: message.role }}
          class={cn("group flex flex-col gap-1", message.role === "user" ? "items-end" : "items-start")}
        >
          {#if message.role === "assistant" && message.reasoning}
            <div class="w-full max-w-[80%] pl-9">
              <ThinkingBlock
                reasoning={message.reasoning}
                streaming={isStreaming && index === messages.length - 1 && !message.content}
              />
            </div>
          {/if}
          <div class={cn("flex items-end gap-3 max-w-[80%]", message.role === "user" ? "justify-end" : "justify-start")}>
            {#if message.role === "assistant"}
              <ModelAvatar family={message.model_family ?? selectedFamily} />
            {/if}

            <div class={cn(
              "min-w-0 rounded-2xl px-4 py-2.5 text-base leading-relaxed wrap-break-word",
              message.role === "user"
                ? "bg-primary text-primary-foreground rounded-br-md"
                : "bg-muted text-foreground rounded-bl-md",
              isEditing && "opacity-60 outline-2 outline-dashed outline-offset-2 outline-primary/60",
              isFailed && "opacity-60 outline-2 outline-dashed outline-offset-2 outline-destructive/60"
            )}>
              <Typewriter
                text={message.content}
                animate={message.role === "assistant" && isStreaming && index === messages.length - 1}
              >
                {#snippet children(revealed)}
                  <Markdown
                    content={revealed}
                    class={message.role === "user" ? "markdown-on-primary" : ""}
                  />
                {/snippet}
              </Typewriter>
              {#if message.role === "assistant" && isStreaming && index === messages.length - 1}
                <span class="inline-block w-1.5 h-4 ml-0.5 bg-foreground/70 animate-pulse rounded-sm align-text-bottom"></span>
              {/if}
            </div>

            {#if message.role === "user"}
              <UserAvatar />
            {/if}
          </div>

          <div class={cn(
            "flex items-center gap-0.5 transition-opacity",
            isFailed || isEditing
              ? "opacity-100"
              : "opacity-0 group-hover:opacity-80 focus-within:opacity-80",
            message.role === "user" ? "justify-end pr-9" : "flex-row-reverse justify-end pl-9"
          )}>
            {#if isFailed}
              <Button
                variant="ghost"
                size="icon"
                onclick={() => onRetry?.()}
                class="size-7 rounded-full text-destructive hover:text-destructive"
                title="Not sent — try again"
                aria-label="Not sent — try again"
              >
                <Icon icon="fa6-solid:rotate-right" class="size-3.5 shrink-0" />
              </Button>
            {/if}
            {#if message.id === latestUserMessageId && message.id !== failedMessageId}
              <EditButton
                editing={isEditing}
                disabled={isStreaming && !isEditing}
                onEdit={() => chatboxStore.startEditing(message.id, message.content)}
                onCancel={() => chatboxStore.stopEditing()}
                tooltipAlign="end"
              />
            {/if}
            <CopyButton
              text={message.content}
              tooltipAlign={message.role === "user" ? "end" : "start"}
            />
            <DeleteButton
              onDelete={() => requestDelete(message.id)}
              disabled={!canDeleteMessage(message)}
              tooltipAlign={message.role === "user" ? "end" : "start"}
            />
          </div>
        </div>
      {/each}

      {#if isStreaming && messages[messages.length - 1]?.role === "user"}
        <div in:bubbleIn={{ role: "assistant" }} class="flex gap-3 justify-start">
          <ModelAvatar family={selectedFamily} />
          <div class="bg-muted rounded-2xl rounded-bl-md px-4 py-3">
            <Icon icon="fa6-solid:spinner" class="size-4 animate-spin text-muted-foreground" />
          </div>
        </div>
      {/if}

      </div>
    </ScrollArea>
  </div>

  <Dialog open={deleteDialogOpen} onOpenChange={(details: { open: boolean }) => { if (!details.open) cancelDelete(); }}>
    <DialogBackdrop />
    <DialogPositioner>
      <DialogContent class="w-full max-w-md">
        <div class="flex flex-col gap-4 p-6">
          <div class="flex items-start gap-3">
            <span class="mt-0.5 flex size-9 shrink-0 items-center justify-center rounded-full bg-destructive/10 text-destructive">
              <Icon icon="fa6-solid:triangle-exclamation" class="size-4.5" />
            </span>
            <div class="min-w-0 flex-1 space-y-1">
              <DialogTitle>Delete message?</DialogTitle>
              <DialogDescription>
                This permanently deletes this message{deleteCount > 1 ? ` and the ${deleteCount - 1} message${deleteCount - 1 === 1 ? "" : "s"} after it` : ""}. This action can't be undone.
              </DialogDescription>
            </div>
          </div>
          <div class="flex items-center justify-end gap-2">
            <Button variant="ghost" size="sm" disabled={isDeleting} onclick={cancelDelete}>
              Cancel
            </Button>
            <Button variant="destructive" size="sm" disabled={isDeleting} onclick={confirmDelete}>
              {#if isDeleting}
                <Icon icon="fa6-solid:spinner" class="size-3.5 animate-spin" />
              {/if}
              Delete
            </Button>
          </div>
        </div>
      </DialogContent>
    </DialogPositioner>
  </Dialog>
{/if}
