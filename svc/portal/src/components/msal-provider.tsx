"use client";

import type { PublicClientApplication } from "@azure/msal-browser";
import { MsalProvider as AzureMsalProvider } from "@azure/msal-react";
import { useEffect, useState } from "react";
import { initializeMsal } from "@/lib/msal";

export function MsalProvider({ children }: { children: React.ReactNode }) {
  const [msalInstance, setMsalInstance] =
    useState<PublicClientApplication | null>(null);

  useEffect(() => {
    initializeMsal().then((i) => {
      setMsalInstance(i);
    });
  }, []);

  if (!msalInstance) {
    return null;
  }

  return (
    <AzureMsalProvider instance={msalInstance}>{children}</AzureMsalProvider>
  );
}
