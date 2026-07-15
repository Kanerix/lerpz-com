<script lang="ts">
import ModelSelector from "$lib/components/model-selector/ModelSelector.svelte";
import { EnhanceButton } from "$lib/components/prompt";
import ClapperAspectRatio from "./ClapperAspectRatio.svelte";
import ClapperDuration from "./ClapperDuration.svelte";
import { clapperStore } from "./clapper.store.svelte.js";
import { getClapperContext } from "./clapper-context.svelte.js";

let {
    enhance,
    isPending,
    isEnhancePending,
}: {
    enhance?: (prompt: string) => Promise<string>;
    isPending: boolean;
    isEnhancePending: boolean;
} = $props();

const clapper = getClapperContext();

async function handleEnhance() {
    if (!clapperStore.prompt.trim() || !enhance) return;
    const newPrompt = await enhance(clapperStore.prompt);
    clapperStore.setPrompt(newPrompt);
}
</script>

<div class="flex flex-wrap items-center gap-2">
  <ModelSelector
    models={clapper.models}
    isModelsLoading={clapper.isModelsLoading}
    value={clapperStore.model}
    onSelect={(value) => clapperStore.setModel(value)}
    getAnchorRect={() =>
      clapperStore.clapperAnchor?.getBoundingClientRect() ?? null}
  />
  <ClapperAspectRatio disabled={isPending} />
  <ClapperDuration disabled={isPending} />

  <!-- Enhance -->
  <div class="ml-auto">
    <EnhanceButton
      loading={isEnhancePending}
      disabled={isPending || !clapperStore.prompt.trim() || !enhance}
      onclick={handleEnhance}
    />
  </div>
</div>
