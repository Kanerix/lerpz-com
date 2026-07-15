<script lang="ts">
import type { Snippet } from "svelte";
import { getAiContext } from "$lib/ai/context.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ChatView from "$lib/components/chatbox/ChatView.svelte";

let { children }: { children: Snippet } = $props();
const ai = getAiContext();

$effect(() => {
    ai.resetChat();
    chatboxStore.stopEditing();
});
</script>

<ChatView messages={ai.chatMessages} isStreaming={ai.isChatStreaming} error={ai.chatError} onRetry={ai.retryChat} />
