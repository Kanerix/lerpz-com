"use client";

import { Popover as PopoverPrimitive } from "@base-ui/react/popover";
import { cn } from "@lerpz/ui/lib/utils";
import * as React from "react";

const Popover = PopoverPrimitive.Root;

const PopoverTrigger = PopoverPrimitive.Trigger;

const PopoverClose = PopoverPrimitive.Close;

const PopoverPortal = PopoverPrimitive.Portal;

const PopoverBackdrop = React.forwardRef<
  HTMLDivElement,
  PopoverPrimitive.Backdrop.Props
>(({ className, ...props }, ref) => (
  <PopoverPrimitive.Backdrop
    ref={ref}
    data-slot="popover-backdrop"
    className={cn("fixed inset-0 z-50", className)}
    {...props}
  />
));
PopoverBackdrop.displayName = "PopoverBackdrop";

function PopoverPositioner({
  className,
  side = "bottom",
  sideOffset = 8,
  align = "center",
  alignOffset = 0,
  ...props
}: PopoverPrimitive.Positioner.Props) {
  return (
    <PopoverPrimitive.Positioner
      data-slot="popover-positioner"
      side={side}
      sideOffset={sideOffset}
      align={align}
      alignOffset={alignOffset}
      className={cn("isolate z-50", className)}
      {...props}
    />
  );
}

function PopoverContent({
  className,
  children,
  ...props
}: PopoverPrimitive.Popup.Props) {
  return (
    <PopoverPrimitive.Popup
      data-slot="popover-content"
      className={cn(
        "bg-popover text-popover-foreground ring-foreground/5 rounded-2xl shadow-2xl ring-1 outline-hidden",
        "data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95",
        "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        "relative isolate z-50 origin-(--transform-origin) duration-100",
        className,
      )}
      {...props}
    >
      {children}
    </PopoverPrimitive.Popup>
  );
}

function PopoverArrow({ className, ...props }: PopoverPrimitive.Arrow.Props) {
  return (
    <PopoverPrimitive.Arrow
      data-slot="popover-arrow"
      className={cn(
        "fill-popover stroke-border/50 -translate-y-px stroke-[0.5px] drop-shadow-[0_1px_0_hsl(var(--border)/0.1)]",
        "[&>path:last-child]:stroke-popover",
        className,
      )}
      {...props}
    />
  );
}

function PopoverTitle({ className, ...props }: PopoverPrimitive.Title.Props) {
  return (
    <PopoverPrimitive.Title
      data-slot="popover-title"
      className={cn("text-base font-medium", className)}
      {...props}
    />
  );
}

function PopoverDescription({
  className,
  ...props
}: PopoverPrimitive.Description.Props) {
  return (
    <PopoverPrimitive.Description
      data-slot="popover-description"
      className={cn("text-muted-foreground text-sm", className)}
      {...props}
    />
  );
}

export {
  Popover,
  PopoverArrow,
  PopoverBackdrop,
  PopoverClose,
  PopoverContent,
  PopoverDescription,
  PopoverPortal,
  PopoverPositioner,
  PopoverTitle,
  PopoverTrigger,
};
