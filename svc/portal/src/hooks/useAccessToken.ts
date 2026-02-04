"use client";

import { useAccount, useMsal } from "@azure/msal-react";
import { useEffect, useState } from "react";
import { loginRequest } from "@/lib/msal-config";

export function useAccessToken() {
  const { instance, accounts } = useMsal();
  const account = useAccount(accounts[0] || undefined);
  const [token, setToken] = useState<string | null>(null);

  useEffect(() => {
    if (!account) return;

    instance
      .acquireTokenSilent({
        ...loginRequest,
        account,
      })
      .then((response) => setToken(response.accessToken))
      .catch((error) => {
        console.error("Token acquisition failed, triggering login:", error);
        instance.loginRedirect(loginRequest);
      });
  }, [account, instance]);

  return token;
}
