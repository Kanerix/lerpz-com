<script lang="ts">
import type { Snippet } from "svelte";
import { getAiContext } from "$lib/ai/context.svelte.js";
import ChatView from "$lib/components/chatbox/ChatView.svelte";

let { children }: { children: Snippet } = $props();
const ai = getAiContext();

$effect(() => {
    ai.resetChat();
});
</script>

<ChatView messages={ai.chatMessages} isStreaming={ai.isChatStreaming} error={ai.chatError} />
