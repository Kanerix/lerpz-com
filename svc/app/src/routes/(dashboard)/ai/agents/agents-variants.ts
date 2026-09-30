import { cva } from "class-variance-authority";

/** Memory option picker for the agent form. */
export const optionCardVariants = cva(
    "flex h-auto items-start justify-start gap-3 whitespace-normal rounded-xl border p-4 text-left transition-colors",
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
