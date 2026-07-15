<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { cn } from "@lerpz/ui/lib/utils";

let {
    onDelete,
    disabled = false,
    class: className = "",
}: {
    /** Called when the user confirms the delete action. */
    onDelete?: () => void;
    disabled?: boolean;
    class?: string;
} = $props();
</script>

<Tooltip side="bottom" align="end">
  <TooltipTrigger>
    {#snippet asChild(getProps)}
      <Button
        {...getProps()}
        variant="ghost"
        size="icon-xs"
        {disabled}
        onclick={() => onDelete?.()}
        aria-label="Delete message"
        class={cn("text-muted-foreground hover:text-destructive", className)}
      >
        <Icon icon="fa6-regular:trash-can" class="size-3.5 shrink-0" />
      </Button>
    {/snippet}
  </TooltipTrigger>
  <TooltipContent>
    <p>Delete message</p>
  </TooltipContent>
</Tooltip>
