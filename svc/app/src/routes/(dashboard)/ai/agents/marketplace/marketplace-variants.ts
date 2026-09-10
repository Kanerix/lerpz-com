import { cva } from "class-variance-authority";

/** Category filter pill above the template grid. */
export const categoryPillVariants = cva(
    "rounded-full border px-3 py-1 text-sm transition-colors",
    {
        variants: {
            active: {
                true: "border-primary bg-primary text-primary-foreground",
                false: "border-border text-muted-foreground hover:bg-muted",
            },
        },
        defaultVariants: { active: false },
    },
);
