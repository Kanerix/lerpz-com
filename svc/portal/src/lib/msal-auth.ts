"use client";

import { PublicClientApplication } from "@azure/msal-browser";
import { env } from "./env";
import { loginRequest, msalConfig } from "./msal-config";

export const msalInstance = new PublicClientApplication(msalConfig);

export async function getAccessToken(): Promise<string | null> {
  if (!msalInstance) {
    console.error("MSAL instance not initialized");
    return null;
  }

  const accounts = msalInstance.getAllAccounts();
  if (accounts.length === 0) {
    console.warn("No accounts found, user needs to sign in");
    return null;
  }

  const request = {
    ...loginRequest,
    account: accounts[0],
  };

  try {
    const response = await msalInstance.acquireTokenSilent(request);
    return response.accessToken;
  } catch (e) {
    console.error(e)
    // await msalInstance.loginRedirect({
    //   ...loginRequest,
    //   scopes: [env.NEXT_PUBLIC_ENTRA_ID_SCOPE],
    // });
    return null;
  }
}
