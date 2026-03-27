"use client";

import { useTheme } from "next-themes";
import {
  MdCheckCircle,
  MdError,
  MdInfo,
  MdLoop,
  MdWarning,
} from "react-icons/md";
import { Toaster as Sonner, type ToasterProps } from "sonner";

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = "system" } = useTheme();

  return (
    <Sonner
      theme={theme as ToasterProps["theme"]}
      className="toaster group"
      icons={{
        success: <MdCheckCircle className="size-4" />,
        info: <MdInfo className="size-4" />,
        warning: <MdWarning className="size-4" />,
        error: <MdError className="size-4" />,
        loading: <MdLoop className="size-4 animate-spin" />,
      }}
      style={
        {
          "--normal-bg": "var(--popover)",
          "--normal-text": "var(--popover-foreground)",
          "--normal-border": "var(--border)",
          "--border-radius": "var(--radius)",
        } as React.CSSProperties
      }
      toastOptions={{
        classNames: {
          toast: "cn-toast",
        },
      }}
      {...props}
    />
  );
};

export { Toaster };
