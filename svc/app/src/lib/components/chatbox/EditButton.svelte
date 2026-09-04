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
    onCancel,
    editing = false,
    disabled = false,
    class: className = "",
    tooltipAlign = "end",
}: {
    /** Called when the user wants to start editing this message. */
    onEdit?: () => void;
    /** Called when the user wants to cancel the in-progress edit. */
    onCancel?: () => void;
    /** Whether this message is currently being edited. */
    editing?: boolean;
    disabled?: boolean;
    class?: string;
    /** Alignment of the tooltip relative to the button. */
    tooltipAlign?: "start" | "center" | "end";
} = $props();

const label = $derived(editing ? "Cancel edit" : "Edit message");
</script>

<Tooltip side="bottom" align={tooltipAlign}>
  <TooltipTrigger>
    {#snippet asChild(getProps)}
      <Button
        {...getProps()}
        variant="ghost"
        size="icon-xs"
        {disabled}
        onclick={() => (editing ? onCancel?.() : onEdit?.())}
        aria-label={label}
        class={cn("text-muted-foreground", className)}
      >
        <Icon
          icon={editing ? "fa6-solid:xmark" : "fa6-regular:pen-to-square"}
          class="size-3.5 shrink-0"
        />
      </Button>
    {/snippet}
  </TooltipTrigger>
  <TooltipContent>
    <p>{label}</p>
  </TooltipContent>
</Tooltip>
