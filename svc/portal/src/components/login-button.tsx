"use client";

import type { IPublicClientApplication } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
import { Button } from "@lerpz/ui/components/button";
import { loginRequest } from "@/lib/msal";

export default function LoginButton() {
  const { instance } = useMsal();

  function handleLogin(instance: IPublicClientApplication) {
    instance.loginRedirect(loginRequest).catch((e) => {
      console.error(e);
    });
  }

  return <Button onClick={() => handleLogin(instance)}>Login</Button>;
}
