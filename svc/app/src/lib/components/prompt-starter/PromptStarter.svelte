<script lang="ts">
import Icon from "@iconify/svelte";
import { cn } from "@lerpz/ui/lib/utils";
import ModelAvatar from "$lib/components/avatar/ModelAvatar.svelte";
import type { PromptExample } from "./prompt-starter.js";

let {
    family = null,
    title,
    description,
    examples,
    onSelect,
    class: className,
}: {
    /** Model family behind the avatar, so the starter matches the chosen model. */
    family?: string | null;
    /** Headline, such as "Start a conversation". */
    title: string;
    /** Supporting line beneath the title. */
    description: string;
    /** Cards offered under the copy. Picking one loads its prompt. */
    examples: PromptExample[];
    /** Called with the picked example's prompt. */
    onSelect: (prompt: string) => void;
    class?: string;
} = $props();
</script>

<div
  class={cn(
    "flex flex-col items-center justify-center px-4 text-center",
    className,
  )}
>
  <div class="flex w-full max-w-xl flex-col items-center gap-6">
    <div class="flex flex-col items-center gap-3">
      <ModelAvatar {family} size="lg" />
      <h2 class="text-xl font-semibold tracking-tight">{title}</h2>
      <p class="text-sm text-muted-foreground max-w-sm">{description}</p>
    </div>
    <div class="grid w-full gap-2 sm:grid-cols-2">
      {#each examples as example (example.title)}
        <button
          type="button"
          onclick={() => onSelect(example.prompt)}
          class="group flex cursor-pointer flex-col gap-1 rounded-xl border border-border bg-card/40 p-3 text-left transition-colors hover:border-primary/40 hover:bg-accent"
        >
          <span class="flex items-center gap-2 text-sm font-medium">
            <Icon icon={example.icon} class="size-3.5 text-muted-foreground transition-colors group-hover:text-primary" />
            {example.title}
          </span>
          <span class="line-clamp-2 text-xs text-muted-foreground">{example.prompt}</span>
        </button>
      {/each}
    </div>
  </div>
</div>
