<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import type { Model } from "$lib/ai/models.svelte.js";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ModelSelector from "$lib/components/model-selector/ModelSelector.svelte";
import { REASONING_KEY } from "$lib/components/model-selector/reasoning.js";
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

<div class="flex gap-2">
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

  <div class="flex gap-x-2 ml-auto">
    <!-- Enhance -->
    <Tooltip>
      <TooltipTrigger>
        <Button
          variant="ghost"
          size="icon"
          aria-label="Enhance prompt"
          disabled={isPending || !chatboxStore.prompt.trim() || !enhance}
          onclick={handleEnhance}
        >
          {#if isEnhancePending}
            <Icon icon="fa6-solid:spinner" class="animate-spin" />
          {:else}
            <Icon icon="fa6-solid:wand-magic-sparkles" />
          {/if}
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Enhance the prompt!</p></TooltipContent>
    </Tooltip>
  </div>
</div>
