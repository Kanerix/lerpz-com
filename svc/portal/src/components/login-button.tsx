"use client";

import { InteractionStatus } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
import { Button } from "@lerpz/ui/components/button";
import { loginRequest } from "@/lib/msal/config";

export default function LoginButton() {
  const { instance, inProgress } = useMsal();

  const handleLogin = async () => {
    try {
      await instance.loginRedirect(loginRequest);
    } catch (error) {
      console.error("Login failed:", error);
    }
  };

  return (
    <Button
      onClick={handleLogin}
      disabled={inProgress !== InteractionStatus.None}
    >
      Sign In with Microsoft
    </Button>
  );
}
