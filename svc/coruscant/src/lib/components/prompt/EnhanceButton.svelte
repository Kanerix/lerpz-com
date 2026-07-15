<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";

let {
    loading = false,
    disabled = false,
    onclick,
}: {
    loading?: boolean;
    disabled?: boolean;
    onclick?: () => void;
} = $props();
</script>

<Tooltip>
  <TooltipTrigger>
    <Button
      variant="ghost"
      size="icon"
      aria-label="Enhance prompt"
      {disabled}
      {onclick}
    >
      {#if loading}
        <Icon icon="fa6-solid:spinner" class="animate-spin" />
      {:else}
        <Icon icon="fa6-solid:wand-magic-sparkles" />
      {/if}
    </Button>
  </TooltipTrigger>
  <TooltipContent><p>Enhance the prompt!</p></TooltipContent>
</Tooltip>
