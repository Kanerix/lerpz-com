"use client";

import { InteractionStatus } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
import { Button } from "@lerpz/ui/components/button";
import type { ComponentProps } from "react";
import { loginRequest } from "@/lib/msal-config";

type LoginButtonProps = Omit<ComponentProps<typeof Button>, "disabled" | "onClick">;

export default function LoginButton({
  children,
  className,
  ...props
}: LoginButtonProps) {
  const { instance, inProgress } = useMsal();

  const handleLogin = async () => {
    try {
      await instance.loginRedirect(loginRequest);
    } catch {
      console.error("Was unable to login to your account");
    }
  };

  return (
    <Button
      onClick={handleLogin}
      disabled={inProgress !== InteractionStatus.None}
      {...props}
    >
      {children}
    </Button>
  );
}
