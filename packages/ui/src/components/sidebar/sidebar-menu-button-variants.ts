import { cva, type VariantProps } from "class-variance-authority";

export const sidebarMenuButtonVariants = cva(
    [
        "peer/menu-button flex w-full items-center gap-2 overflow-hidden",
        "group-data-[state=collapsed]:justify-center",
        "rounded-lg p-2 text-left text-sm outline-none ring-sidebar-ring",
        "cursor-pointer",
        "transition-[width,height,padding] focus-visible:ring-2",
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        "active:bg-sidebar-accent active:text-sidebar-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-50",
        "aria-disabled:pointer-events-none aria-disabled:opacity-50",
        "group-has-[[data-sidebar=menu-action]]/menu-item:pr-8",
        "data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium",
        "data-[active=true]:text-sidebar-accent-foreground",
    ],
    {
        variants: {
            size: {
                default: "h-8 text-sm",
                sm: "h-7 text-xs",
                lg: "h-12 text-sm",
            },
        },
        defaultVariants: { size: "default" },
    },
);

export type SidebarMenuButtonVariants = VariantProps<
    typeof sidebarMenuButtonVariants
>;
