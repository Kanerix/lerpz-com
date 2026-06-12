<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@lerpz/ui/components/select";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { toast } from "svelte-sonner";
import { chatboxStore } from "$lib/components/chatbox/chatbox.store.svelte.js";
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

async function handleCameraCapture() {
    let stream: MediaStream | null = null;
    try {
        stream = await navigator.mediaDevices.getUserMedia({
            video: { facingMode: "environment" },
        });
    } catch (err) {
        const msg =
            err instanceof DOMException && err.name === "NotAllowedError"
                ? "Camera access was denied."
                : "Could not access the camera.";
        toast.error("Camera error", { description: msg });
        return;
    }
    try {
        const video = document.createElement("video");
        video.srcObject = stream;
        video.setAttribute("playsinline", "true");
        await video.play();
        await new Promise((r) => setTimeout(r, 300));
        const canvas = document.createElement("canvas");
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        const ctx = canvas.getContext("2d");
        if (!ctx) {
            toast.error("Camera error", {
                description: "Canvas context failed.",
            });
            return;
        }
        ctx.drawImage(video, 0, 0);
        const blob = await new Promise<Blob | null>((r) =>
            canvas.toBlob(r, "image/png"),
        );
        if (!blob) {
            toast.error("Camera error", {
                description: "Failed to capture photo.",
            });
            return;
        }
        const file = new File(
            [blob],
            `camera-${new Date().toISOString().replace(/[:.]/g, "-")}.png`,
            { type: "image/png" },
        );
        chatboxStore.addUploadedImages([file]);
    } finally {
        stream?.getTracks().forEach((t) => {
            t.stop();
        });
    }
}

async function handleEnhance() {
    if (!chatboxStore.prompt || !enhance) return;
    const newPrompt = await enhance(chatboxStore.prompt);
    chatboxStore.setPrompt(newPrompt);
}

const modelItems = $derived(
    chatbox.models.map((m) => ({ value: m.value ?? "", label: m.label })),
);
</script>

<div class="flex gap-2">
  <!-- Model selector -->
  <Select
    items={modelItems}
    value={chatboxStore.model}
    onValueChange={(v) => chatboxStore.setModel(v)}
  >
    <SelectTrigger size="sm" class="gap-1.5">
      {#if chatbox.isModelsLoading}
        <Icon icon="mdi:loading" class="animate-spin" />
      {:else}
        <Icon icon="mdi:robot-outline" />
      {/if}
      <SelectValue placeholder="Select model" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        {#each modelItems as item}
          <SelectItem value={item.value}>{item.label}</SelectItem>
        {/each}
      </SelectGroup>
    </SelectContent>
  </Select>

  <div class="flex gap-x-2 ml-auto">
    <!-- Camera -->
    <Tooltip>
      <TooltipTrigger>
        <Button
          class="hidden sm:flex"
          variant="outline"
          size="icon"
          aria-label="Take photo"
          disabled={isPending}
          onclick={handleCameraCapture}
        >
          <Icon icon="mdi:camera-outline" />
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Take a photo using camera</p></TooltipContent>
    </Tooltip>

    <!-- Upload images -->
    <Tooltip>
      <TooltipTrigger>
        <Button
          class="hidden sm:flex"
          variant="outline"
          size="icon"
          aria-label="Add images"
          disabled={isPending}
          onclick={() => fileInputEl?.click()}
        >
          <input bind:this={fileInputEl} type="file" accept="image/*" multiple class="hidden" onchange={handleFileChange} />
          <Icon icon="mdi:image-plus-outline" />
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Add images to your prompt</p></TooltipContent>
    </Tooltip>

    <!-- Enhance -->
    <Tooltip>
      <TooltipTrigger>
        <Button
          variant="outline"
          size="icon"
          aria-label="Enhance prompt"
          disabled={isPending || !chatboxStore.prompt.trim() || !enhance}
          onclick={handleEnhance}
        >
          {#if isEnhancePending}
            <Icon icon="mdi:loading" class="animate-spin" />
          {:else}
            <Icon icon="mdi:auto-fix-high" />
          {/if}
        </Button>
      </TooltipTrigger>
      <TooltipContent><p>Enhance the prompt!</p></TooltipContent>
    </Tooltip>
  </div>
</div>
