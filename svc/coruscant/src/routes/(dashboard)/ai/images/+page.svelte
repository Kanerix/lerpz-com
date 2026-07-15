<script lang="ts">
import { getAiContext } from "$lib/ai/context.svelte.js";
import Easel from "$lib/components/easel/Easel.svelte";

const ai = getAiContext();
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
      <div class="text-center text-muted-foreground">
        <p class="text-base font-medium">Describe an image to bring it to life</p>
        <p class="text-sm">
          Write a prompt below, pick a model and aspect ratio, then generate.
        </p>
      </div>
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
