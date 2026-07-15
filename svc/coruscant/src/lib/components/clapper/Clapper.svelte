<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import { filterModelsByModality, type Model } from "$lib/ai/models.svelte.js";
import {
    PromptComposer,
    PromptInputRow,
    PromptSubmitButton,
} from "$lib/components/prompt";
import ClapperSettings from "./ClapperSettings.svelte";
import ClapperStatusBar from "./ClapperStatusBar.svelte";
import { clapperStore } from "./clapper.store.svelte.js";
import type { ClapperSubmitArgs } from "./clapper-context.svelte.js";
import { setClapperContext } from "./clapper-context.svelte.js";

let {
    onSubmit,
    onEnhance,
    models = [],
    isModelsLoading = false,
    isGenerating = false,
    error = null,
    class: className = "",
}: {
    onSubmit?: (args: ClapperSubmitArgs) => void | Promise<void>;
    onEnhance?: (prompt: string) => Promise<string>;
    models?: Model[];
    isModelsLoading?: boolean;
    isGenerating?: boolean;
    error?: string | null;
    class?: string;
} = $props();

let isSubmitPending = $state(false);
let isEnhancePending = $state(false);

const isPending = $derived(isSubmitPending || isEnhancePending || isGenerating);

// Only video-generation models make sense in the clapper.
const availableModels = $derived(filterModelsByModality(models, "video"));

let textareaEl = $state<HTMLTextAreaElement | null>(null);
let cardEl = $state<HTMLDivElement | null>(null);

// Focus once the input is rendered (it mounts with the composer's intro).
$effect(() => {
    textareaEl?.focus();
});

$effect(() => {
    clapperStore.setClapperAnchor(cardEl);
});

// Grow the textarea with its content, up to a sensible ceiling.
function autosize() {
    const el = textareaEl;
    if (!el) return;
    el.style.height = "auto";
    el.style.height = `${Math.min(el.scrollHeight, 240)}px`;
}

$effect(() => {
    // Re-run autosize when the prompt is changed programmatically (e.g. enhance).
    clapperStore.prompt;
    autosize();
});

// Default the model to the first available one, matching the easel behaviour.
$effect(() => {
    if (isModelsLoading || availableModels.length === 0) return;
    if (clapperStore.model !== null) return;
    const first = availableModels[0];
    if (first) clapperStore.setModel(first.value);
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

setClapperContext({
    get models() {
        return availableModels;
    },
    get isModelsLoading() {
        return isModelsLoading;
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
    const trimmed = clapperStore.prompt.trim();
    if (!trimmed || isPending) return;
    const args: ClapperSubmitArgs = {
        prompt: trimmed,
        model: clapperStore.model,
        aspectRatio: clapperStore.aspectRatio,
        duration: clapperStore.duration,
    };
    isSubmitPending = true;
    try {
        if (onSubmit) await onSubmit(args);
        clapperStore.setPrompt("");
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

function handleKeydown(e: KeyboardEvent) {
    // Enter submits; Shift+Enter (or Ctrl/Cmd+Enter) inserts a newline.
    if (e.key === "Enter" && !e.shiftKey) {
        e.preventDefault();
        submit();
    }
}

function handleInput(e: Event) {
    clapperStore.setPrompt((e.target as HTMLTextAreaElement).value);
    autosize();
}
</script>

<PromptComposer bind:cardEl class={className}>
  {#snippet statusBar()}
    <ClapperStatusBar {isGenerating} {error} />
  {/snippet}
  <PromptInputRow>
    <textarea
      bind:this={textareaEl}
      value={clapperStore.prompt}
      oninput={handleInput}
      onkeydown={handleKeydown}
      disabled={isPending}
      placeholder="Describe the video you want to create…"
      rows="1"
      class={cn(
        "grow resize-none self-stretch bg-transparent px-1 py-1.5 text-sm",
        "leading-relaxed placeholder:text-muted-foreground focus:outline-none",
        "disabled:opacity-60",
      )}
    ></textarea>
    <PromptSubmitButton
      loading={isPending}
      disabled={isPending || !clapperStore.prompt.trim()}
      label="Generate video"
      onclick={submit}
    />
  </PromptInputRow>
  <ClapperSettings {enhance} {isPending} {isEnhancePending} />
</PromptComposer>

<style>
textarea {
    max-height: 240px;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
}
</style>
