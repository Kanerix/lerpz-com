<script lang="ts">
import { getAiContext } from "$lib/ai/context.svelte.js";
import Easel from "$lib/components/easel/Easel.svelte";
import { easelStore } from "$lib/components/easel/easel.store.svelte.js";
import type { PromptExample } from "$lib/components/prompt-starter/index.js";
import { PromptStarter } from "$lib/components/prompt-starter/index.js";

const ai = getAiContext();

// Fallback family for the starter avatar: whichever image model is selected.
const selectedFamily = $derived(
    ai.models.find((m) => m.value === easelStore.model)?.family ?? null,
);

const EXAMPLE_PROMPTS: PromptExample[] = [
    {
        icon: "fa6-solid:mountain-sun",
        title: "Set a scene",
        prompt: "A misty pine forest at sunrise, soft light through the trees.",
    },
    {
        icon: "fa6-solid:palette",
        title: "Pick a style",
        prompt: "A city skyline in flat vector art, bold shapes and few colours.",
    },
    {
        icon: "fa6-solid:box",
        title: "Shoot a product",
        prompt: "A matte ceramic coffee cup on a concrete table, studio lighting.",
    },
    {
        icon: "fa6-solid:user-astronaut",
        title: "Draw a character",
        prompt: "A friendly robot librarian sorting books, warm storybook style.",
    },
];
</script>

<div class="mx-auto flex h-full w-full max-w-5xl flex-col gap-4 py-4">
  <!-- Results -->
  <div class="flex min-h-0 flex-1 items-center justify-center overflow-y-auto">
    {#if ai.generatedImage}
      <img
        src={ai.generatedImage}
        alt="Generated"
        class="max-h-full max-w-full rounded-2xl object-contain shadow-lg"
      />
    {:else if ai.isImageLoading}
      <p class="text-sm text-muted-foreground">Generating image…</p>
    {:else}
      <PromptStarter
        family={selectedFamily}
        title="Create an image"
        description="Describe an image below to generate one, or pick an example to get started. Finished images are saved to your gallery."
        examples={EXAMPLE_PROMPTS}
        onSelect={(prompt) => easelStore.setPrompt(prompt)}
      />
    {/if}
  </div>

  <!-- Prompt -->
  <Easel
    onSubmit={async (args) => {
      ai.startImage(args.prompt, {
        model: args.model,
        amount: args.count,
      });
    }}
    onEnhance={ai.enhanceImage}
    isGenerating={ai.isImageLoading}
    error={ai.imageError}
    models={ai.models}
    isModelsLoading={ai.isModelsLoading}
  />
</div>
