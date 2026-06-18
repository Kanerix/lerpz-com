<script lang="ts">
import Icon from "@iconify/svelte";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
</script>

<div class="flex justify-center gap-x-1 mb-2">
  {#each chatboxStore.uploadedImages as image (image.id)}
    <div class="group relative h-16 w-16 overflow-hidden rounded-xl border bg-background shadow-sm">
      <button
        type="button"
        onclick={() => chatboxStore.removeUploadedImage(image.id)}
        class="absolute right-1 top-1 z-10 h-6 w-6 flex items-center justify-center rounded-full bg-background/80 text-foreground shadow-sm opacity-0 ring-1 ring-border transition-opacity group-hover:opacity-100"
        aria-label="Remove image"
      >
        <Icon icon="fa6-solid:xmark" class="h-3 w-3" />
      </button>
      <img
        src={image.previewUrl}
        alt={image.file.name || "Uploaded image"}
        class="h-full w-full object-cover"
      />
    </div>
  {/each}
</div>
