<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import { filterModelsByModality, type Model } from "$lib/ai/models.svelte.js";
import {
    PromptComposer,
    PromptInputRow,
    PromptSubmitButton,
} from "$lib/components/prompt";
import EaselSettings from "./EaselSettings.svelte";
import EaselStatusBar from "./EaselStatusBar.svelte";
import { easelStore } from "./easel.store.svelte.js";
import type { EaselSubmitArgs } from "./easel-context.svelte.js";
import { setEaselContext } from "./easel-context.svelte.js";

let {
    onSubmit,
    onEnhance,
    models = [],
    isModelsLoading = false,
    isGenerating = false,
    error = null,
    class: className = "",
}: {
    onSubmit?: (args: EaselSubmitArgs) => void | Promise<void>;
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

// Only image-generation models make sense in the easel.
const availableModels = $derived(filterModelsByModality(models, "image"));

let textareaEl = $state<HTMLTextAreaElement | null>(null);
let cardEl = $state<HTMLDivElement | null>(null);

// Focus once the input is rendered (it mounts with the composer's intro).
$effect(() => {
    textareaEl?.focus();
});

$effect(() => {
    easelStore.setEaselAnchor(cardEl);
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
    easelStore.prompt;
    autosize();
});

// Default the model to the first available one, matching the chatbox behaviour.
$effect(() => {
    if (isModelsLoading || availableModels.length === 0) return;
    // Keep the persisted selection while it's still available; otherwise fall
    // back to the first model.
    const current = easelStore.model;
    if (current && availableModels.some((m) => m.value === current)) return;
    const first = availableModels[0];
    if (first) easelStore.setModel(first.value);
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

setEaselContext({
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
    const trimmed = easelStore.prompt.trim();
    if (!trimmed || isPending) return;
    const args: EaselSubmitArgs = {
        prompt: trimmed,
        model: easelStore.model,
        aspectRatio: easelStore.aspectRatio,
        count: easelStore.count,
    };
    isSubmitPending = true;
    try {
        if (onSubmit) await onSubmit(args);
        easelStore.setPrompt("");
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
    easelStore.setPrompt((e.target as HTMLTextAreaElement).value);
    autosize();
}
</script>

<PromptComposer bind:cardEl class={className}>
  {#snippet statusBar()}
    <EaselStatusBar {isGenerating} {error} />
  {/snippet}
  <PromptInputRow>
    <textarea
      bind:this={textareaEl}
      value={easelStore.prompt}
      oninput={handleInput}
      onkeydown={handleKeydown}
      disabled={isPending}
      placeholder="Describe the image you want to create…"
      rows="1"
      class={cn(
        "grow resize-none self-stretch bg-transparent px-1 py-1.5 text-sm",
        "leading-relaxed placeholder:text-muted-foreground focus:outline-none",
        "disabled:opacity-60",
      )}
    ></textarea>
    <PromptSubmitButton
      loading={isSubmitPending || isGenerating}
      disabled={isPending || !easelStore.prompt.trim()}
      label="Generate image"
      onclick={submit}
    />
  </PromptInputRow>
  <EaselSettings {enhance} {isPending} {isEnhancePending} />
</PromptComposer>

<style>
textarea {
    max-height: 240px;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
}
</style>
