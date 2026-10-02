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
import { msalStore } from "$lib/auth/msal.svelte.js";
import Chatbox from "$lib/components/chatbox/Chatbox.svelte";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import { DEFAULT_REASONING_LEVEL } from "$lib/components/model-selector/reasoning.js";
import { notificationStore } from "$lib/notifications/notifications.svelte.js";
import { queryKeys } from "$lib/query/keys.js";

let { children }: { children: Snippet } = $props();

const queryClient = useQueryClient();
const accountKey = $derived(msalStore.accountKey);
let chatAccountKey: string | null = null;
let imageAccountKey: string | null = null;
let videoAccountKey: string | null = null;

const chat = createChat({
    onSaved: (convId) => {
        const submissionAccount = chatAccountKey;
        if (!submissionAccount) return;
        void queryClient.invalidateQueries({
            queryKey: queryKeys.chats.all(submissionAccount),
            refetchType: submissionAccount === accountKey ? "active" : "none",
        });
        if (submissionAccount === accountKey) {
            void goto(`/ai/chats/${convId}`, { replaceState: true });
        }
    },
});

const image = createImage({
    onDone: () => {
        const submissionAccount = imageAccountKey;
        if (!submissionAccount) return;
        void queryClient.invalidateQueries({
            queryKey: queryKeys.images.all(submissionAccount),
            refetchType: submissionAccount === accountKey ? "active" : "none",
        });
    },
});
// Video renders run as background jobs, so surface their outcome through the
// notification bell. The user may have moved to another AI page by the time a
// render finishes. `lastVideoPrompt` gives the notification a meaningful body.
let lastVideoPrompt = "";
const video = createVideo({
    onDone: () => {
        const submissionAccount = videoAccountKey;
        if (!submissionAccount) return;
        void queryClient.invalidateQueries({
            queryKey: queryKeys.videos.all(submissionAccount),
            refetchType: submissionAccount === accountKey ? "active" : "none",
        });
        if (submissionAccount !== accountKey) return;
        notificationStore.add({
            title: "Video ready",
            body: lastVideoPrompt || "Your video finished generating.",
            icon: "fa6-solid:clapperboard",
            href: "/ai/videos",
        });
    },
    onError: (message) => {
        if (!videoAccountKey || videoAccountKey !== accountKey) return;
        notificationStore.add({
            title: "Video generation failed",
            body: message,
            icon: "fa6-solid:triangle-exclamation",
            href: "/ai/videos",
        });
    },
});
const modelsHook = createModels();

function sendChat(prompt: string, options?: Parameters<typeof chat.send>[1]) {
    chatAccountKey = accountKey;
    chat.send(prompt, options);
}

function editChat(
    prompt: string,
    options?: Parameters<typeof chat.editLatest>[1],
) {
    chatAccountKey = accountKey;
    chat.editLatest(prompt, options);
}

function retryChat() {
    chatAccountKey = accountKey;
    chat.retry();
}

function enterConversation(
    id: string,
    messages?: Parameters<typeof chat.enterConversation>[1],
) {
    chatAccountKey = accountKey;
    chat.enterConversation(id, messages);
}

function startImage(
    prompt: string,
    options?: Parameters<typeof image.start>[1],
) {
    imageAccountKey = accountKey;
    image.start(prompt, options);
}

function startVideo(
    prompt: string,
    options?: Parameters<typeof video.start>[1],
) {
    videoAccountKey = accountKey;
    // Cap the remembered prompt so a long one doesn't bloat the notification.
    lastVideoPrompt = prompt.length > 140 ? `${prompt.slice(0, 139)}…` : prompt;
    video.start(prompt, options);
}

let stateAccountKey: string | null | undefined;
$effect(() => {
    const currentAccountKey = accountKey;
    if (currentAccountKey === stateAccountKey) return;

    stateAccountKey = currentAccountKey;
    chatAccountKey = null;
    imageAccountKey = null;
    videoAccountKey = null;
    lastVideoPrompt = "";
    chat.reset();
    image.reset();
    video.reset();
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
    retryChat,
    enterConversation,
    removeChatMessagesFrom: chat.removeMessagesFrom,
    sendChat,
    editChat,
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
    startImage,
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
    get isVideoBackgrounded() {
        return video.isBackgrounded;
    },
    get videoStartedAt() {
        return video.startedAt;
    },
    stopVideo: video.stop,
    resetVideo: video.reset,
    startVideo,
    backgroundVideo: video.background,
    foregroundVideo: video.foreground,
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
        editChat(args.prompt, { model: args.model, reasoning, family });
        chatboxStore.stopEditing();
      } else {
        sendChat(args.prompt, { model: args.model, reasoning, family });
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
