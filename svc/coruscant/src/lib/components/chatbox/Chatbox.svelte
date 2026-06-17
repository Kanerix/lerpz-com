<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { Card, CardContent } from "@lerpz/ui/components/card";
import { untrack } from "svelte";
import type { Model } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ChatboxSettings from "./ChatboxSettings.svelte";
import ChatStatusBar from "./ChatStatusBar.svelte";
import MarkdownEditor from "./MarkdownEditor.svelte";
import type {
    ChatboxMode,
    ChatboxSubmitArgs,
} from "./chatbox-context.svelte.js";
import {
    setChatboxContext,
} from "./chatbox-context.svelte.js";
import ImageShelf from "./ImageShelf.svelte";

const placeholders: Record<ChatboxMode, string> = {
    chat: "Send a message!",
    image: "Describe your image!",
    video: "Describe your video!",
};

let {
    onSubmit,
    onEnhance,
    models = [],
    isModelsLoading = false,
    isStreaming = false,
    isThinking = false,
    isSaved = false,
    error = null,
    loadModels = async () => {},
    defaultMode = "chat",
}: {
    onSubmit?: (args: ChatboxSubmitArgs) => void | Promise<void>;
    onEnhance?: (prompt: string) => Promise<string>;
    models?: Model[];
    isModelsLoading?: boolean;
    isStreaming?: boolean;
    isThinking?: boolean;
    isSaved?: boolean;
    error?: string | null;
    loadModels?: (mode?: string) => Promise<void>;
    defaultMode?: ChatboxMode;
} = $props();

let mode = $state<ChatboxMode>(untrack(() => defaultMode));
let showSettings = $state(true);
let isSubmitPending = $state(false);
let isEnhancePending = $state(false);

const isPending = $derived(isSubmitPending || isEnhancePending || isStreaming);

let cardEl = $state<HTMLDivElement | null>(null);

$effect(() => {
    chatboxStore.setChatboxAnchor(cardEl);
});

$effect(() => {
    if (isModelsLoading || models.length === 0) return;
    if (chatboxStore.model !== null) return;
    const first = models[0];
    if (first) chatboxStore.setModel(first.value);
});

$effect(() => {
    if (!isPending) return;
    const handler = (e: BeforeUnloadEvent) => {
        e.preventDefault();
        return "";
    };
    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
});

setChatboxContext({
    get mode() {
        return mode;
    },
    setMode: (m) => {
        mode = m;
    },
    get showSettings() {
        return showSettings;
    },
    setShowSettings: (v) => {
        showSettings = v;
    },
    allowImageUploads: true,
    get models() {
        return models;
    },
    get isModelsLoading() {
        return isModelsLoading;
    },
    get loadModels() {
        return loadModels;
    },
    submit,
    get isSubmitPending() {
        return isSubmitPending;
    },
    enhance,
    get isEnhancePending() {
        return isEnhancePending;
    },
    get isPending() {
        return isPending;
    },
});

async function submit() {
    const trimmed = chatboxStore.prompt.trim();
    if (!trimmed || isPending) return;
    const args: ChatboxSubmitArgs = {
        prompt: trimmed,
        mode,
        model: chatboxStore.model,
        modelSettings: chatboxStore.getModelSettings(
            chatboxStore.model ?? undefined,
        ),
        images: chatboxStore.uploadedImages,
    };
    isSubmitPending = true;
    try {
        if (onSubmit) await onSubmit(args);
        chatboxStore.setPrompt("");
        chatboxStore.clearUploadedImages();
    } finally {
        isSubmitPending = false;
    }
}

async function enhance(prompt: string): Promise<string> {
    if (!onEnhance) return prompt;
    isEnhancePending = true;
    try {
        return await onEnhance(prompt);
    } finally {
        isEnhancePending = false;
    }
}

function handleEnter() {
    if (!isPending && chatboxStore.prompt.trim()) submit();
}
</script>

<aside class="absolute bottom-0 left-1/2 -translate-x-1/2 w-full max-w-[750px] p-4">
  <ImageShelf />
  <ChatStatusBar {isThinking} {isSaved} {error} />
  <div bind:this={cardEl}>
    <Card class="rounded-4xl">
      <CardContent class="flex flex-col gap-3 p-3">
        <div class="flex items-end gap-4">
          <MarkdownEditor
            value={chatboxStore.prompt}
            onChange={(md) => chatboxStore.setPrompt(md)}
            onEnter={handleEnter}
            disabled={isPending}
            placeholder={placeholders[mode]}
            autofocus
            class="grow self-stretch px-1 py-1.5"
          />
          <Button
            size="icon"
            onclick={submit}
            disabled={isPending || !chatboxStore.prompt.trim()}
            aria-label="Send prompt"
          >
            {#if isPending}
              <Icon icon="mdi:loading" class="animate-spin" />
            {:else}
              <Icon icon="mdi:arrow-up" />
            {/if}
          </Button>
        </div>
        <!-- Settings row -->
        {#if showSettings}
          <ChatboxSettings {enhance} {isPending} {isEnhancePending} />
        {/if}
      </CardContent>
    </Card>
  </div>
</aside>
