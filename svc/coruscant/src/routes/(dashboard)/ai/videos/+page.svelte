<script lang="ts">
import { onDestroy } from "svelte";
import { getAiContext } from "$lib/ai/context.svelte.js";
import Clapper from "$lib/components/clapper/Clapper.svelte";
import { clapperStore } from "$lib/components/clapper/clapper.store.svelte.js";
import type { ClapperSubmitArgs } from "$lib/components/clapper/clapper-context.svelte.js";
import VideoStage from "$lib/components/video-stage/VideoStage.svelte";

const ai = getAiContext();

let lastArgs = $state<ClapperSubmitArgs | null>(null);

function generate(args: ClapperSubmitArgs) {
    lastArgs = args;
    ai.startVideo(args.prompt, {
        model: args.model,
        aspectRatio: args.aspectRatio,
        duration: args.duration,
    });
}

onDestroy(() => ai.backgroundVideo());
</script>

<div class="mx-auto flex h-full min-h-0 w-full max-w-5xl flex-col">
  <div class="flex min-h-0 w-full flex-1 px-4 pt-4">
    <VideoStage
      video={ai.generatedVideo}
      isLoading={ai.isVideoLoading}
      isBackgrounded={ai.isVideoBackgrounded}
      startedAt={ai.videoStartedAt}
      error={ai.videoError}
      aspectRatio={clapperStore.aspectRatio}
      onRetry={lastArgs ? () => generate(lastArgs!) : undefined}
      onDismiss={ai.resetVideo}
      onBackground={ai.backgroundVideo}
      onForeground={ai.foregroundVideo}
    />
  </div>

  <!-- Prompt -->
  <Clapper
    class="shrink-0"
    onSubmit={async (args) => generate(args)}
    onEnhance={ai.enhanceVideo}
    isGenerating={ai.isVideoLoading}
    error={ai.videoError}
    models={ai.models}
    isModelsLoading={ai.isModelsLoading}
  />
</div>
