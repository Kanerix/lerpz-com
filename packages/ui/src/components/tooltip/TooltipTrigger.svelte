<script lang="ts">
import { Tooltip } from "@ark-ui/svelte/tooltip";
import type { Snippet } from "svelte";
import type { HTMLAttributes, SvelteHTMLElements } from "svelte/elements";
import { cn } from "../../lib/utils.js";

// `PropsFn` is declared in @ark-ui/svelte's internal types but not re-exported,
// so we mirror its definition here for the `asChild` snippet.
type PropsFn<T extends keyof SvelteHTMLElements> = (
    props?: SvelteHTMLElements[T],
) => HTMLAttributes<HTMLElement>;

let {
    class: className = "",
    children,
    asChild,
    ...rest
}: {
    class?: string;
    children?: Snippet;
    asChild?: Snippet<[PropsFn<"button">]>;
    [key: string]: unknown;
} = $props();
</script>

<Tooltip.Trigger
    data-slot="tooltip-trigger"
    class={cn("cursor-pointer", className)}
    {asChild}
    {...rest}
>
  {@render children?.()}
</Tooltip.Trigger>
