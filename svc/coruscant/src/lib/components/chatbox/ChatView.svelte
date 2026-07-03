<script lang="ts">
import Icon from "@iconify/svelte";
import { Avatar, AvatarFallback } from "@lerpz/ui/components/avatar";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Typewriter } from "@lerpz/ui/components/typewriter";
import { cn } from "@lerpz/ui/lib/utils";
import { cubicOut } from "svelte/easing";
import type { ConversationMessage } from "$lib/api/models/index.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import CopyButton from "./CopyButton.svelte";
import Markdown from "./Markdown.svelte";
import ThinkingBlock from "./ThinkingBlock.svelte";

let {
    messages = [],
    isStreaming = false,
    error = null,
}: {
    messages: ConversationMessage[];
    isStreaming: boolean;
    error: string | null;
} = $props();

// Distance (px) from the bottom within which we consider the
// user to be "at the bottom" and therefore following the stream.
const BOTTOM_THRESHOLD = 64;

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
</script>

{#if messages.length === 0}
  <div class="flex flex-col items-center justify-center text-center gap-4">
    <div class="flex flex-col items-center gap-3">
      <div class="bg-muted rounded-full p-4">
        <Icon icon="fa6-solid:robot" class="size-8 text-muted-foreground" />
      </div>
      <h2 class="text-xl font-semibold tracking-tight">Start a conversation</h2>
      <p class="text-sm text-muted-foreground max-w-sm">
        Type a message below to begin chatting. Your conversation will be saved automatically.
      </p>
    </div>
  </div>
{:else}
  <div class="relative h-full w-full">
    <ScrollArea bind:viewportRef orientation="vertical" class="h-full w-full">
      <div bind:this={contentRef} class="mx-auto max-w-6xl flex flex-col gap-4" style="padding-bottom: {chatboxStore.chatboxHeight + 24}px">
      {#each messages as message, index (message.id)}
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
              <Avatar size="sm" class="shrink-0">
                <AvatarFallback>
                  <Icon icon="fa6-solid:robot" class="size-3.5" />
                </AvatarFallback>
              </Avatar>
            {/if}

            <div class={cn(
              "min-w-0 rounded-2xl px-4 py-2.5 text-base leading-relaxed break-words",
              message.role === "user"
                ? "bg-primary text-primary-foreground rounded-br-md"
                : "bg-muted text-foreground rounded-bl-md"
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
              <Avatar size="sm" class="shrink-0">
                <AvatarFallback>
                  <Icon icon="fa6-solid:user" class="size-3.5" />
                </AvatarFallback>
              </Avatar>
            {/if}
          </div>

          <div class={cn(
            "opacity-0 transition-opacity group-hover:opacity-80 focus-within:opacity-80",
            message.role === "user" ? "pr-9" : "pl-9"
          )}>
            <CopyButton text={message.content} />
          </div>
        </div>
      {/each}

      {#if isStreaming && messages[messages.length - 1]?.role === "user"}
        <div in:bubbleIn={{ role: "assistant" }} class="flex gap-3 justify-start">
          <Avatar size="sm" class="shrink-0">
            <AvatarFallback>
              <Icon icon="fa6-solid:robot" class="size-3.5" />
            </AvatarFallback>
          </Avatar>
          <div class="bg-muted rounded-2xl rounded-bl-md px-4 py-3">
            <Icon icon="fa6-solid:spinner" class="size-4 animate-spin text-muted-foreground" />
          </div>
        </div>
      {/if}

      {#if error}
        <div class="mx-auto text-sm text-destructive bg-destructive/10 rounded-lg px-4 py-2">
          {error}
        </div>
      {/if}

      </div>
    </ScrollArea>
  </div>
{/if}
