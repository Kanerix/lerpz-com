<script lang="ts">
import type { Model } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ModelSelector from "$lib/components/model-selector/ModelSelector.svelte";
import { REASONING_KEY } from "$lib/components/model-selector/reasoning.js";
import { EnhanceButton } from "$lib/components/prompt";
import { getChatboxContext } from "./chatbox-context.svelte.js";

let {
    enhance,
    isPending,
    isEnhancePending,
}: {
    enhance?: (prompt: string) => Promise<string>;
    isPending: boolean;
    isEnhancePending: boolean;
} = $props();

const chatbox = getChatboxContext();

async function handleEnhance() {
    if (!chatboxStore.prompt || !enhance) return;
    const newPrompt = await enhance(chatboxStore.prompt);
    chatboxStore.setPrompt(newPrompt);
}
</script>

<div class="flex flex-wrap items-center gap-2">
  <!-- Model selector -->
  <ModelSelector
    models={chatbox.models}
    isModelsLoading={chatbox.isModelsLoading}
    value={chatboxStore.model}
    onSelect={(value) => chatboxStore.setModel(value)}
    getReasoningLevel={(model: Model) =>
      chatboxStore.getModelSetting(model.value ?? undefined, REASONING_KEY)}
    onReasoningChange={(model: Model, level) => {
      if (model.value) chatboxStore.setModelSetting(model.value, REASONING_KEY, level);
    }}
    getAnchorRect={() =>
      chatboxStore.chatboxAnchor?.getBoundingClientRect() ?? null}
  />

  <!-- Enhance -->
  <div class="ml-auto">
    <EnhanceButton
      loading={isEnhancePending}
      disabled={isPending || !chatboxStore.prompt.trim() || !enhance}
      onclick={handleEnhance}
    />
  </div>
</div>
