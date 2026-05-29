<script lang="ts">
import { getAiContext } from "$lib/ai/context.svelte.js";

const ai = getAiContext();
</script>

<div class="max-w-5xl w-full h-full mx-auto">
  {#if ai.imageError}
    <p class="text-sm text-destructive mb-4">{ai.imageError}</p>
  {/if}
  {#if ai.generatedImage}
    <img src={ai.generatedImage} alt="Generated" class="max-w-full" />
  {:else}
    <p>{ai.isImageLoading ? "Generating image..." : "Describe an image in the chatbox below to generate one!"}</p>
  {/if}
</div>
