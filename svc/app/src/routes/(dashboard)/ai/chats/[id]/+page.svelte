<script lang="ts">
import { Skeleton } from "@lerpz/ui";
import { createQuery } from "@tanstack/svelte-query";
import { browser } from "$app/environment";
import { getAiContext } from "$lib/ai/context.svelte.js";
import { getChat } from "$lib/api/chats/chats.js";
import { msalStore } from "$lib/auth/msal.svelte.js";
import ChatView from "$lib/components/chatbox/ChatView.svelte";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import { ErrorState } from "$lib/components/error-state/index.js";
import { queryKeys } from "$lib/query/keys.js";
import type { PageProps } from "./$types.js";

let { params }: PageProps = $props();

const id = $derived(params.id);
const accountKey = $derived(msalStore.accountKey);
const ai = getAiContext();

// Clear any in-progress edit when the visible conversation changes so a stale
// edit target can't misroute the next send.
$effect(() => {
    void id;
    chatboxStore.stopEditing();
});

const isLive = $derived(ai.conversationId === id && ai.chatMessages.length > 0);

const query = createQuery(() => ({
    queryKey: queryKeys.chats.detail(accountKey, id),
    enabled: browser && accountKey !== null && !isLive,
    queryFn: async ({ signal }: { signal: AbortSignal }) => {
        const response = await getChat(id, { signal });
        if (response.status !== 200) throw response.data;
        return response.data;
    },
    meta: { skipGlobalErrorDialog: true },
}));

$effect(() => {
    if (isLive || ai.conversationId === id) return;
    const conversation = query.data;
    if (!conversation) return;
    ai.enterConversation(id, conversation.messages);
});

const messages = $derived(
    isLive ? ai.chatMessages : (query.data?.messages ?? []),
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
{:else if !isLive && query.isError && !query.data}
  <ErrorState
    class="mx-auto max-w-200"
    title="Couldn't load this chat"
    onRetry={() => query.refetch()}
    retrying={query.isFetching}
  />
{:else}
  <ChatView {messages} isStreaming={isLive && ai.isChatStreaming} error={isLive ? ai.chatError : null} onRetry={ai.retryChat} />
{/if}
