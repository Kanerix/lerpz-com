import { cva } from "class-variance-authority";

/** Image thumbnail in the source picker, one of which is the analysed image. */
export const thumbnailVariants = cva(
    [
        "group relative aspect-square cursor-pointer overflow-hidden rounded-lg",
        "border bg-muted/30 outline-none transition",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring",
    ],
    {
        variants: {
            selected: {
                true: "border-primary ring-2 ring-primary",
                false: "border-border hover:border-primary/50",
            },
        },
        defaultVariants: { selected: false },
    },
);

/** Drop zone for a user-supplied image, highlighted while a file is over it. */
export const dropZoneVariants = cva(
    [
        "flex flex-1 cursor-pointer flex-col items-center justify-center gap-3",
        "rounded-xl border border-dashed px-4 py-16 text-center transition",
    ],
    {
        variants: {
            dragging: {
                true: "border-primary bg-primary/5",
                false: "border-border hover:border-primary/50",
            },
        },
        defaultVariants: { dragging: false },
    },
);
