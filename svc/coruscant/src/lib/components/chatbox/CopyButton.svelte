<script lang="ts">
import Icon from "@iconify/svelte";
import { cn } from "@lerpz/ui/lib/utils";
import { toast } from "svelte-sonner";

let { text = "", class: className = "" }: { text?: string; class?: string } =
    $props();

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

<button
  type="button"
  onclick={copy}
  aria-label={copied ? "Copied to clipboard" : "Copy message"}
  class={cn(
    "inline-flex items-center gap-1 rounded-md px-1.5 py-0.5 text-xs text-muted-foreground transition-colors hover:text-foreground",
    className,
  )}
>
  <Icon icon={copied ? "fa6-solid:check" : "fa6-regular:copy"} class="size-3.5 shrink-0" />
  <span>{copied ? "Copied" : "Copy"}</span>
</button>
