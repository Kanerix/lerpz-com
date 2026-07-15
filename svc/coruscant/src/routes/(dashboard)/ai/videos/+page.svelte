<script lang="ts">
import { getAiContext } from "$lib/ai/context.svelte.js";
import Clapper from "$lib/components/clapper/Clapper.svelte";

const ai = getAiContext();
</script>

<div class="mx-auto flex h-full w-full max-w-5xl flex-col gap-4 py-4">
  <!-- Results -->
  <div class="flex min-h-0 flex-1 items-center justify-center overflow-y-auto">
    {#if ai.generatedVideo}
      <!-- svelte-ignore a11y_media_has_caption -->
      <video
        src={ai.generatedVideo}
        controls
        autoplay
        loop
        class="max-h-full max-w-full rounded-2xl object-contain shadow-lg"
      ></video>
    {:else if ai.isVideoLoading}
      <p class="text-sm text-muted-foreground">Generating video…</p>
    {:else}
      <div class="text-center text-muted-foreground">
        <p class="text-base font-medium">Describe a video to bring it to life</p>
        <p class="text-sm">
          Write a prompt below, pick a model, aspect ratio and duration, then generate.
        </p>
      </div>
    {/if}
  </div>

  <!-- Prompt -->
  <Clapper
    onSubmit={async (args) => {
      ai.startVideo(args.prompt, {
        model: args.model,
        aspectRatio: args.aspectRatio,
        duration: args.duration,
      });
    }}
    onEnhance={async (prompt) => prompt}
    isGenerating={ai.isVideoLoading}
    error={ai.videoError}
    models={ai.models}
    isModelsLoading={ai.isModelsLoading}
  />
</div>
