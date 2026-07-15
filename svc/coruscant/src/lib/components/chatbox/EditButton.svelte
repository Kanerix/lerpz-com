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
    onEdit,
    disabled = false,
    class: className = "",
    tooltipAlign = "end",
}: {
    /** Called when the user wants to edit this message. */
    onEdit?: () => void;
    disabled?: boolean;
    class?: string;
    /** Alignment of the tooltip relative to the button. */
    tooltipAlign?: "start" | "center" | "end";
} = $props();
</script>

<Tooltip side="bottom" align={tooltipAlign}>
  <TooltipTrigger>
    {#snippet asChild(getProps)}
      <Button
        {...getProps()}
        variant="ghost"
        size="icon-xs"
        {disabled}
        onclick={() => onEdit?.()}
        aria-label="Edit message"
        class={cn("text-muted-foreground", className)}
      >
        <Icon icon="fa6-regular:pen-to-square" class="size-3.5 shrink-0" />
      </Button>
    {/snippet}
  </TooltipTrigger>
  <TooltipContent>
    <p>Edit message</p>
  </TooltipContent>
</Tooltip>
