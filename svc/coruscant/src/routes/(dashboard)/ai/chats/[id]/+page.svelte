<script lang="ts">
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery } from "@tanstack/svelte-query";
import { getAiContext } from "$lib/ai/context.svelte.js";
import { getChat } from "$lib/api/chats/chats.js";
import ChatView from "$lib/components/chatbox/ChatView.svelte";
import type { PageProps } from "./$types.js";

let { params }: PageProps = $props();

const id = $derived(params.id);
const ai = getAiContext();

const isLive = $derived(ai.conversationId === id && ai.chatMessages.length > 0);

const query = createQuery(() => ({
    queryKey: [`/api/v1/chats/${id}`],
    queryFn: ({ signal }: { signal: AbortSignal }) => getChat(id, { signal }),
    enabled: !isLive,
}));

$effect(() => {
    if (isLive || ai.conversationId === id) return;
    const resp = query.data;
    if (resp?.status !== 200 || !resp.data) return;
    ai.enterConversation(id, resp.data.messages);
});

const messages = $derived(
    isLive
        ? ai.chatMessages
        : query.data?.status === 200 && query.data.data
          ? query.data.data.messages
          : [],
);
</script>

{#if !isLive && query.isLoading}
  <div class="mx-auto max-w-200 flex flex-col gap-4 pt-4 pb-8 px-4">
    <div class="flex justify-end gap-3">
      <Skeleton class="h-10 w-56 rounded-2xl rounded-br-md" />
      <Skeleton class="size-7 rounded-full shrink-0 mt-1" />
    </div>
    <div class="flex justify-start gap-3">
      <Skeleton class="size-7 rounded-full shrink-0 mt-1" />
      <Skeleton class="h-24 w-80 rounded-2xl rounded-bl-md" />
    </div>
  </div>
{:else if !isLive && query.data?.status !== 200}
  <p class="text-muted-foreground px-2 py-4 text-center text-xs">
    Error: {query.data?.status}
  </p>
{:else}
  <ChatView {messages} isStreaming={isLive && ai.isChatStreaming} error={isLive ? ai.chatError : null} onRetry={ai.retryChat} />
{/if}
