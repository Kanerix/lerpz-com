<script lang="ts">
import type { Snippet } from "svelte";
import { goto } from "$app/navigation";
import { createChat } from "$lib/ai/chat.svelte.js";
import { setAiContext } from "$lib/ai/context.svelte.js";
import { createImageSse } from "$lib/ai/image-sse.svelte.js";
import { createModels } from "$lib/ai/models.svelte.js";
import Chatbox from "$lib/components/chatbox/Chatbox.svelte";

let { children }: { children: Snippet } = $props();

const chat = createChat({
    onSaved: (convId) => {
        goto(`/ai/chats/${convId}`, { replaceState: true });
    },
});

const image = createImageSse();
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
    enterConversation: chat.enterConversation,
    sendChat: chat.send,
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
    get models() {
        return modelsHook.models;
    },
    get isModelsLoading() {
        return modelsHook.isLoading;
    },
    loadModels: modelsHook.loadModels,
});

const isPending = $derived(
    chat.isStreaming || chat.isLoading || image.isLoading,
);
</script>

{@render children()}
<Chatbox
  onSubmit={async (args) => {
    switch (args.mode) {
      case "chat": {
        const model = modelsHook.models.find((m) => m.value === args.model);
        const reasoning = model?.reasoning
          ? (args.modelSettings.reasoning ?? null)
          : null;
        chat.send(args.prompt, { model: args.model, reasoning });
        break;
      }
      case "image": image.start(args.prompt); break;
      case "video": break; // Video generation is not yet available.
    }
  }}
  onEnhance={async (prompt) => prompt}
  isStreaming={isPending}
  isThinking={isPending}
  isSaved={chat.isSaved}
  error={chat.error ?? image.error}
  models={modelsHook.models}
  isModelsLoading={modelsHook.isLoading}
  loadModels={modelsHook.loadModels}
/>
