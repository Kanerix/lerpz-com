<script lang="ts">
import { getAiContext } from "$lib/ai/context.svelte.js";
import Clapper from "$lib/components/clapper/Clapper.svelte";
import type { ClapperSubmitArgs } from "$lib/components/clapper/clapper-context.svelte.js";
import { clapperStore } from "$lib/components/clapper/clapper.store.svelte.js";
import VideoStage from "$lib/components/video-stage/VideoStage.svelte";

const ai = getAiContext();

// Remember the last submission so a failed render can be retried verbatim.
let lastArgs = $state<ClapperSubmitArgs | null>(null);

function generate(args: ClapperSubmitArgs) {
    lastArgs = args;
    ai.startVideo(args.prompt, {
        model: args.model,
        aspectRatio: args.aspectRatio,
        duration: args.duration,
    });
}
</script>

<div class="mx-auto flex h-full w-full max-w-5xl flex-col gap-4 py-4">
  <!-- Results -->
  <div class="flex min-h-0 flex-1 items-center justify-center overflow-y-auto">
    <VideoStage
      video={ai.generatedVideo}
      isLoading={ai.isVideoLoading}
      error={ai.videoError}
      aspectRatio={clapperStore.aspectRatio}
      onRetry={lastArgs ? () => generate(lastArgs!) : undefined}
      onDismiss={ai.resetVideo}
    />
  </div>

  <!-- Prompt -->
  <Clapper
    onSubmit={async (args) => generate(args)}
    onEnhance={ai.enhanceVideo}
    isGenerating={ai.isVideoLoading}
    error={ai.videoError}
    models={ai.models}
    isModelsLoading={ai.isModelsLoading}
  />
</div>
