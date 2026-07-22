import { cva, type VariantProps } from "class-variance-authority";

export const buttonVariants = cva(
    [
        "inline-flex shrink-0 items-center justify-center whitespace-nowrap",
        "rounded-lg border border-transparent bg-clip-padding transition-all",
        "text-sm font-medium select-none outline-none cursor-pointer",
        "[&_svg]:pointer-events-none [&_svg]:shrink-0",
        "[&_svg:not([class*='size-'])]:size-4",
        "focus-visible:border-ring focus-visible:ring-ring/50",
        "focus-visible:ring-[3px]",
        "disabled:pointer-events-none disabled:opacity-50",
        "aria-invalid:border-destructive aria-invalid:ring-destructive/20",
        "aria-invalid:ring-[3px] dark:aria-invalid:ring-destructive/40",
    ],
    {
        variants: {
            variant: {
                default:
                    "bg-primary text-primary-foreground hover:bg-primary/80",
                outline:
                    "border-border bg-input/30 hover:bg-input/50 hover:text-foreground",
                secondary:
                    "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                ghost: "hover:bg-muted hover:text-foreground dark:hover:bg-muted/50",
                destructive:
                    "bg-destructive/10 hover:bg-destructive/20 text-destructive",
                link: "text-primary underline-offset-4 hover:underline",
            },
            size: {
                default: "h-9 gap-1.5 px-3",
                xs: "h-6 gap-1 px-2.5 text-xs [&_svg:not([class*='size-'])]:size-3",
                sm: "h-8 gap-1 px-3",
                lg: "h-10 gap-1.5 px-4",
                icon: "size-9",
                "icon-xs": "size-6 [&_svg:not([class*='size-'])]:size-3",
                "icon-sm": "size-8",
                "icon-lg": "size-10",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    },
);

export type ButtonVariants = VariantProps<typeof buttonVariants>;
