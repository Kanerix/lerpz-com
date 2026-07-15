<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { cubicOut } from "svelte/easing";
import { fly } from "svelte/transition";
import { filterModelsByModality, type Model } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import { ErrorDialog } from "$lib/components/error-dialog";
import {
    PromptComposer,
    PromptInputRow,
    PromptSubmitButton,
} from "$lib/components/prompt";
import ChatboxSettings from "./ChatboxSettings.svelte";
import ChatStatusBar from "./ChatStatusBar.svelte";
import type { ChatboxSubmitArgs } from "./chatbox-context.svelte.js";
import { setChatboxContext } from "./chatbox-context.svelte.js";
import MarkdownEditor from "./MarkdownEditor.svelte";

const placeholder = "Send a message!";

let {
    onSubmit,
    onStop,
    onEnhance,
    models = [],
    isModelsLoading = false,
    isStreaming = false,
    isThinking = false,
    isSaved = false,
    error = null,
    errorValue = null,
    loadModels = async () => {},
}: {
    onSubmit?: (args: ChatboxSubmitArgs) => void | Promise<void>;
    onStop?: () => void;
    onEnhance?: (prompt: string) => Promise<string>;
    models?: Model[];
    isModelsLoading?: boolean;
    isStreaming?: boolean;
    isThinking?: boolean;
    isSaved?: boolean;
    error?: string | null;
    errorValue?: unknown;
    loadModels?: (mode?: string) => Promise<void>;
} = $props();

let showSettings = $state(true);
let isSubmitPending = $state(false);
let isEnhancePending = $state(false);

let errorDialogOpen = $state(false);
let lastShownError: unknown = null;

$effect(() => {
    const current = errorValue;
    if (current == null) {
        lastShownError = null;
        return;
    }
    if (current !== lastShownError) {
        lastShownError = current;
        errorDialogOpen = true;
    }
});

const isPending = $derived(isSubmitPending || isEnhancePending || isStreaming);

// Only text-generation models make sense in the chat surface.
const availableModels = $derived(filterModelsByModality(models, "text"));

let cardEl = $state<HTMLDivElement | null>(null);

$effect(() => {
    const el = cardEl;
    if (!el) return;
    const observer = new ResizeObserver(() => {
        chatboxStore.setChatboxHeight(el.offsetHeight);
    });
    observer.observe(el);
    return () => observer.disconnect();
});

$effect(() => {
    chatboxStore.setChatboxAnchor(cardEl);
});

$effect(() => {
    if (isModelsLoading || availableModels.length === 0) return;
    // Keep the persisted selection while it's still available; otherwise fall
    // back to the first model.
    const current = chatboxStore.model;
    if (current && availableModels.some((m) => m.value === current)) return;
    const first = availableModels[0];
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
        return availableModels;
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

<PromptComposer
  bind:cardEl
  class="absolute bottom-0 left-1/2 -translate-x-1/2"
>
  {#snippet statusBar()}
    <ChatStatusBar
      {isThinking}
      {isSaved}
      {error}
      onShowError={() => (errorDialogOpen = true)}
    />
  {/snippet}
  {#snippet overlay()}
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
  {/snippet}
  <PromptInputRow>
    <MarkdownEditor
      value={chatboxStore.prompt}
      onChange={(md) => chatboxStore.setPrompt(md)}
      onEnter={handleEnter}
      disabled={isPending}
      {placeholder}
      autofocus
      class="grow self-stretch px-1 py-1.5"
    />
    <PromptSubmitButton
      loading={isSubmitPending}
      streaming={isStreaming}
      disabled={isPending || !chatboxStore.prompt.trim()}
      label="Send prompt"
      onclick={submit}
      onStop={onStop}
    />
  </PromptInputRow>
  <!-- Settings row -->
  {#if showSettings}
    <ChatboxSettings {enhance} {isPending} {isEnhancePending} />
  {/if}
</PromptComposer>

<ErrorDialog bind:open={errorDialogOpen} error={errorValue} />
