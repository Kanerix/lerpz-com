"use client";

import {
  InteractionRequiredAuthError,
  PublicClientApplication,
} from "@azure/msal-browser";
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

  try {
    const response = await msalInstance.acquireTokenSilent(loginRequest);
    return response.accessToken;
  } catch (error) {
    if (error instanceof InteractionRequiredAuthError) {
      try {
        const response = await msalInstance.acquireTokenPopup(loginRequest);
        return response.accessToken;
      } catch {
        await msalInstance.loginRedirect(loginRequest);
        return null;
      }
    }

    console.error("Failed to acquire token silently for API:", error);
    return null;
  }
}
