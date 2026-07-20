<script lang="ts">
import { useQueryClient } from "@tanstack/svelte-query";
import type { Snippet } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { createChat } from "$lib/ai/chat.svelte.js";
import { setAiContext } from "$lib/ai/context.svelte.js";
import { enhanceChat, enhanceImage, enhanceVideo } from "$lib/ai/enhance.js";
import { createImage } from "$lib/ai/image.svelte.js";
import { createModels } from "$lib/ai/models.svelte.js";
import { createVideo } from "$lib/ai/video.svelte.js";
import { getListChatsUrl } from "$lib/api/chats/chats.js";
import Chatbox from "$lib/components/chatbox/Chatbox.svelte";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import { DEFAULT_REASONING_LEVEL } from "$lib/components/model-selector/reasoning.js";

let { children }: { children: Snippet } = $props();

const queryClient = useQueryClient();

const chat = createChat({
    onSaved: (convId) => {
        // Refresh the sidebar chat list so the newly created conversation
        // shows up without a manual reload.
        queryClient.invalidateQueries({ queryKey: [getListChatsUrl()] });
        goto(`/ai/chats/${convId}`, { replaceState: true });
    },
});

const image = createImage();
const video = createVideo();
const modelsHook = createModels();

$effect(() => {
    modelsHook.loadModels();
});

setAiContext({
    get chatMessages() {
        return chat.messages;
    },
    get isChatLoading() {
        return chat.isLoading;
    },
    get isChatStreaming() {
        return chat.isStreaming;
    },
    get chatError() {
        return chat.error;
    },
    get conversationId() {
        return chat.conversationId;
    },
    get isChatSaved() {
        return chat.isSaved;
    },
    stopChat: chat.stop,
    resetChat: chat.reset,
    retryChat: chat.retry,
    enterConversation: chat.enterConversation,
    removeChatMessagesFrom: chat.removeMessagesFrom,
    sendChat: chat.send,
    editChat: chat.editLatest,
    get generatedImage() {
        return image.image;
    },
    get isImageLoading() {
        return image.isLoading;
    },
    get isImageDone() {
        return image.isDone;
    },
    get imageError() {
        return image.error;
    },
    stopImage: image.stop,
    resetImage: image.reset,
    startImage: image.start,
    get generatedVideo() {
        return video.video;
    },
    get isVideoLoading() {
        return video.isLoading;
    },
    get isVideoDone() {
        return video.isDone;
    },
    get videoError() {
        return video.error;
    },
    stopVideo: video.stop,
    resetVideo: video.reset,
    startVideo: video.start,
    get models() {
        return modelsHook.models;
    },
    get isModelsLoading() {
        return modelsHook.isLoading;
    },
    loadModels: modelsHook.loadModels,
    enhanceChat,
    enhanceImage,
    enhanceVideo,
});

const isPending = $derived(chat.isStreaming || chat.isLoading);

// The chatbox is only used to drive conversations, so it should be limited to
// the chat area rather than every AI sub-page (images, videos, agents, …). The
// history page is a management view, so it opts out too.
const showChatbox = $derived(
    page.url.pathname.startsWith("/ai/chats") &&
        !page.url.pathname.startsWith("/ai/chats/history"),
);
</script>

{@render children()}
{#if showChatbox}
  <Chatbox
    onSubmit={async (args) => {
      const model = modelsHook.models.find((m) => m.value === args.model);
      const reasoning = model?.reasoning
        ? (args.modelSettings.reasoning ?? DEFAULT_REASONING_LEVEL)
        : null;
      const family = model?.family || null;
      if (chatboxStore.editingMessageId) {
        chat.editLatest(args.prompt, { model: args.model, reasoning, family });
        chatboxStore.stopEditing();
      } else {
        chat.send(args.prompt, { model: args.model, reasoning, family });
      }
    }}
    onEnhance={enhanceChat}
    onStop={chat.stop}
    isStreaming={isPending}
    isThinking={isPending}
    isSaved={chat.isSaved}
    error={chat.error}
    errorValue={chat.errorValue}
    models={modelsHook.models}
    isModelsLoading={modelsHook.isLoading}
    loadModels={modelsHook.loadModels}
  />
{/if}
