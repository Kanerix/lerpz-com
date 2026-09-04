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
    "Mixing the pigments…",
    "Priming the canvas…",
    "Sketching the composition…",
    "Chasing the light…",
    "Blending the palette…",
    "Rendering the details…",
    "Dreaming in pixels…",
    "Adjusting the brushstrokes…",
    "Summoning the muse…",
    "Developing the film…",
];
</script>

<PromptStatusBar busy={isGenerating} {quotes} {error} />
