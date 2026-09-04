<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { mode, toggleMode } from "mode-watcher";
</script>

<Tooltip>
  <TooltipTrigger>
    {#snippet asChild(getProps)}
      <Button
        {...getProps()}
        variant="ghost"
        size="icon"
        onclick={toggleMode}
        aria-label="Toggle theme"
      >
        {#if $mode === "dark"}
          <Icon icon="fa6-solid:sun" class="h-5 w-5" />
        {:else}
          <Icon icon="fa6-solid:moon" class="h-5 w-5" />
        {/if}
        <span class="sr-only">Toggle theme</span>
      </Button>
    {/snippet}
  </TooltipTrigger>
  <TooltipContent>
    <p>Toggle theme</p>
  </TooltipContent>
</Tooltip>
