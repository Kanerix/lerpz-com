<script lang="ts">
import { cva, type VariantProps } from "class-variance-authority";
import type { Snippet } from "svelte";
import { cn } from "../../lib/utils.js";

const badgeVariants = cva(
    [
        "inline-flex items-center px-2.5 py-0.5 text-xs font-semibold",
        "rounded-full border transition-colors",
    ],
    {
        variants: {
            variant: {
                default:
                    "border-transparent bg-primary text-primary-foreground",
                secondary:
                    "border-transparent bg-secondary text-secondary-foreground",
                destructive:
                    "border-transparent bg-destructive text-destructive-foreground",
                outline: "text-foreground",
            },
        },
        defaultVariants: { variant: "default" },
    },
);

let {
    class: className = "",
    variant = "default",
    children,
    ...rest
}: {
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
} & VariantProps<typeof badgeVariants> = $props();
</script>

<span
  data-slot="badge"
  class={cn(badgeVariants({ variant, className }))}
  {...rest}
>
  {@render children?.()}
</span>
