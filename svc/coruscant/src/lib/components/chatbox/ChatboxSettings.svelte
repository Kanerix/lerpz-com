<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
import ModelSelector from "./ModelSelector.svelte";

let {
    enhance,
    isPending,
    isEnhancePending,
}: {
    enhance?: (prompt: string) => Promise<string>;
    isPending: boolean;
    isEnhancePending: boolean;
} = $props();

let fileInputEl = $state<HTMLInputElement | null>(null);

function handleFileChange(e: Event) {
    const files = (e.target as HTMLInputElement).files;
    if (!files?.length) return;
    const imageFiles = Array.from(files).filter((f) =>
        f.type.startsWith("image/"),
    );
    if (imageFiles.length) chatboxStore.addUploadedImages(imageFiles);
    (e.target as HTMLInputElement).value = "";
}

async function handleEnhance() {
    if (!chatboxStore.prompt || !enhance) return;
    const newPrompt = await enhance(chatboxStore.prompt);
    chatboxStore.setPrompt(newPrompt);
}
</script>

<div class="flex gap-2">
  <!-- Model selector -->
  <ModelSelector />

  <div class="flex gap-x-2 ml-auto">
    <!-- Upload images -->
    <Tooltip>
      <TooltipTrigger>
        <Button
          class="hidden sm:flex"
          variant="ghost"
          size="icon"
          aria-label="Add images"
          disabled={isPending}
          onclick={() => fileInputEl?.click()}
        >
          <input bind:this={fileInputEl} type="file" accept="image/*" multiple class="hidden" onchange={handleFileChange} />
          <Icon icon="fa6-regular:image" />
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Add images to your prompt</p></TooltipContent>
    </Tooltip>

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
