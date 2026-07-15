<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@lerpz/ui/components/tooltip";
import { cn } from "@lerpz/ui/lib/utils";
import { toast } from "svelte-sonner";

let {
    text = "",
    class: className = "",
    tooltipAlign = "end",
}: {
    text?: string;
    class?: string;
    /** Alignment of the tooltip relative to the button. */
    tooltipAlign?: "start" | "center" | "end";
} = $props();

let copied = $state(false);
let timeout: ReturnType<typeof setTimeout> | undefined;

async function copy() {
    try {
        if (!navigator.clipboard) {
            throw new Error(
                "Clipboard access requires a secure (HTTPS) context.",
            );
        }
        await navigator.clipboard.writeText(text);
        copied = true;
        clearTimeout(timeout);
        timeout = setTimeout(() => {
            copied = false;
        }, 1500);
    } catch (error) {
        const reason =
            error instanceof Error ? error.message : "Unknown error.";
        toast.error("Couldn't copy to clipboard", { description: reason });
    }
}
</script>

<Tooltip side="bottom" align={tooltipAlign}>
  <TooltipTrigger>
    {#snippet asChild(getProps)}
      <Button
        {...getProps()}
        variant="ghost"
        size="icon-xs"
        onclick={copy}
        aria-label={copied ? "Copied to clipboard" : "Copy message"}
        class={cn("text-muted-foreground", className)}
      >
        <Icon icon={copied ? "fa6-solid:check" : "fa6-regular:copy"} class="size-3.5 shrink-0" />
      </Button>
    {/snippet}
  </TooltipTrigger>
  <TooltipContent>
    <p>{copied ? "Copied!" : "Copy message"}</p>
  </TooltipContent>
</Tooltip>
