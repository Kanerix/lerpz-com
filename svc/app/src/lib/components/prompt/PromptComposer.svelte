<script lang="ts">
import { cn } from "@lerpz/ui/lib/utils";
import { onMount, type Snippet } from "svelte";
import { cubicOut } from "svelte/easing";
import PromptCard from "./PromptCard.svelte";

// Shared shell that gives every prompt composer (chat, image, video) the same
// entrance animation, width and card sizing. Consumers supply the status bar,
// an optional overlay (e.g. a floating "follow" button) and the card contents.
let {
    class: className = "",
    cardEl = $bindable(null),
    statusBar,
    overlay,
    children,
}: {
    class?: string;
    /**
     * The card element, exposed so consumers can anchor popovers to it or
     * observe its size.
     */
    cardEl?: HTMLDivElement | null;
    /** Status line rendered above the card (errors, saved, busy quotes …). */
    statusBar?: Snippet;
    /** Optional content layered over the card (e.g. a floating button). */
    overlay?: Snippet;
    /** The card contents (input row + settings). */
    children?: Snippet;
} = $props();

// Gate rendering on mount so the intro transition plays on first paint.
let mounted = $state(false);
onMount(() => {
    mounted = true;
});

// Matches the chatbox intro: rise up and gently scale in.
function composerIn(_node: Element) {
    return {
        duration: 550,
        easing: cubicOut,
        css: (t: number, u: number) =>
            `opacity: ${t};
             transform: translateY(${u * 28}px) scale(${0.96 + t * 0.04});`,
    };
}
</script>

{#if mounted}
  <div in:composerIn class={cn("mx-auto w-full max-w-5xl p-4", className)}>
    {@render statusBar?.()}
    <div bind:this={cardEl} class="relative">
      {@render overlay?.()}
      <PromptCard>
        {@render children?.()}
      </PromptCard>
    </div>
  </div>
{/if}
