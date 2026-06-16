<script lang="ts">
import type { Snippet } from "svelte";
import type {
    HTMLAnchorAttributes,
    HTMLButtonAttributes,
} from "svelte/elements";
import { cn } from "../../lib/utils.js";
import { type ButtonVariants, buttonVariants } from "./button-variants.js";

type ButtonProps = HTMLButtonAttributes &
    ButtonVariants & {
        href?: undefined;
        children?: Snippet;
    };

type AnchorProps = HTMLAnchorAttributes &
    ButtonVariants & {
        href: string;
        children?: Snippet;
    };

let {
    class: className = "",
    variant = "default",
    size = "default",
    href,
    children,
    ...rest
}: ButtonProps | AnchorProps = $props();
</script>

{#if href}
    <a
        data-slot="button"
        {href}
        class={cn(buttonVariants({ variant, size, className }))}
        {...rest as HTMLAnchorAttributes}
    >
        {@render children?.()}
    </a>
{:else}
    <button
        data-slot="button"
        class={cn(buttonVariants({ variant, size, className }))}
        {...rest as HTMLButtonAttributes}
    >
        {@render children?.()}
    </button>
{/if}
