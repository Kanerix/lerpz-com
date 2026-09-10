import { cva } from "class-variance-authority";

/** Picker card used by the agent form, where one of a pair is chosen. */
export const optionCardVariants = cva(
    "flex items-start gap-3 rounded-xl border p-4 text-left transition-colors",
    {
        variants: {
            selected: {
                true: "border-primary bg-primary/5",
                false: "border-border hover:bg-muted/50",
            },
        },
        defaultVariants: { selected: false },
    },
);
