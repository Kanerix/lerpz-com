<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { Card, CardContent } from "@lerpz/ui/components/card";
import { onMount } from "svelte";
import { cubicOut } from "svelte/easing";
import { fly } from "svelte/transition";
import type { Model } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ChatboxSettings from "./ChatboxSettings.svelte";
import ChatStatusBar from "./ChatStatusBar.svelte";
import type { ChatboxSubmitArgs } from "./chatbox-context.svelte.js";
import { setChatboxContext } from "./chatbox-context.svelte.js";
import MarkdownEditor from "./MarkdownEditor.svelte";

const placeholder = "Send a message!";

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
} = $props();

let showSettings = $state(true);
let isSubmitPending = $state(false);
let isEnhancePending = $state(false);

const isPending = $derived(isSubmitPending || isEnhancePending || isStreaming);

let cardEl = $state<HTMLDivElement | null>(null);
let mounted = $state(false);

onMount(() => {
    mounted = true;
});

$effect(() => {
    const el = cardEl;
    if (!el) return;
    const observer = new ResizeObserver(() => {
        chatboxStore.setChatboxHeight(el.offsetHeight);
    });
    observer.observe(el);
    return () => observer.disconnect();
});

function chatboxIn(_node: Element) {
    return {
        duration: 550,
        easing: cubicOut,
        css: (t: number, u: number) =>
            `opacity: ${t};
             transform: translateY(${u * 28}px) scale(${0.96 + t * 0.04});`,
    };
}

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
    get showSettings() {
        return showSettings;
    },
    setShowSettings: (v) => {
        showSettings = v;
    },
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
        model: chatboxStore.model,
        modelSettings: chatboxStore.getModelSettings(
            chatboxStore.model ?? undefined,
        ),
    };
    isSubmitPending = true;
    try {
        if (onSubmit) await onSubmit(args);
        chatboxStore.setPrompt("");
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

{#if mounted}
<aside
  in:chatboxIn
  class="absolute bottom-0 left-1/2 -translate-x-1/2 w-full max-w-5xl p-4 bg-opacity-0"
>
  <ChatStatusBar {isThinking} {isSaved} {error} />
  <div bind:this={cardEl} class="relative">
    {#if chatboxStore.followButtonVisible}
      <div
        transition:fly={{ y: 12, duration: 200, easing: cubicOut }}
        class="absolute bottom-full left-1/2 z-10 mb-7 -translate-x-1/2"
      >
        <Button
          variant="outline"
          size="sm"
          onclick={() => chatboxStore.followAgentHandler?.()}
          class="rounded-full bg-background/90 shadow-lg backdrop-blur"
        >
          <Icon icon="fa6-solid:arrow-down" class="size-3.5" />
          Follow agent
        </Button>
      </div>
    {/if}
    <Card class="rounded-4xl bg-sidebar text-sidebar-foreground border-sidebar-border">
      <CardContent class="flex flex-col gap-2 p-4">
        <div class="flex items-end gap-4">
          <MarkdownEditor
            value={chatboxStore.prompt}
            onChange={(md) => chatboxStore.setPrompt(md)}
            onEnter={handleEnter}
            disabled={isPending}
            {placeholder}
            autofocus
            class="grow self-stretch px-1 py-1.5"
          />
          <Button
            variant="ghost"
            size="icon"
            onclick={submit}
            disabled={isPending || !chatboxStore.prompt.trim()}
            aria-label="Send prompt"
          >
            {#if isPending}
              <Icon icon="fa6-solid:spinner" class="animate-spin" />
            {:else}
              <Icon icon="fa6-solid:arrow-up" />
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
{/if}
