<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { slide } from "svelte/transition";
import Markdown from "./Markdown.svelte";

let {
    reasoning = "",
    streaming = false,
}: {
    reasoning: string;
    streaming?: boolean;
} = $props();

let userToggled = $state<boolean | null>(null);
const open = $derived(userToggled ?? streaming);

const contentId = `thinking-${Math.random().toString(36).slice(2)}`;
</script>

<div class="flex flex-col text-muted-foreground">
  <Button
    variant="ghost"
    size="xs"
    onclick={() => (userToggled = !open)}
    aria-expanded={open}
    aria-controls={contentId}
    class="w-fit gap-2 rounded-md font-medium text-muted-foreground"
  >
    <Icon icon="fa6-solid:lightbulb" class="size-3.5" />
    <span>Thinking{streaming ? "..." : ""}</span>
    <Icon
      icon="fa6-solid:chevron-down"
      class="size-3 transition-transform duration-200 {open ? 'rotate-180' : ''}"
    />
  </Button>

  {#if open}
    <div
      id={contentId}
      transition:slide={{ duration: 200 }}
      class="border-l border-border ml-4.5 pl-2 text-xs leading-relaxed"
    >
      <Markdown content={reasoning} />
    </div>
  {/if}
</div>
