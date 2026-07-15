<script lang="ts">
import { PromptStatusBar } from "$lib/components/prompt";

let {
    isGenerating = false,
    error = null,
}: {
    isGenerating?: boolean;
    error?: string | null;
} = $props();

const quotes = [
    "Storyboarding the scene…",
    "Rolling the camera…",
    "Rendering the frames…",
    "Setting the keyframes…",
    "Interpolating motion…",
    "Lighting the set…",
    "Directing the pixels…",
    "Cutting the reel…",
    "Syncing the timeline…",
    "Adding the finishing touches…",
];
</script>

<PromptStatusBar busy={isGenerating} {quotes} {error} />
