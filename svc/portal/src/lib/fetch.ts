"use client";

import type { AccountInfo } from "@azure/msal-browser";
import { loginRequest, msalInstance } from "@/lib/msal";

async function getAccessToken(account: AccountInfo) {
  const result = await msalInstance.acquireTokenSilent({
    ...loginRequest,
    account,
  });

  return result.accessToken;
}

export async function authenticatedFetch(
  url: string,
  options: RequestInit = {},
  account: AccountInfo,
): Promise<Response> {
  const accessToken = await getAccessToken(account);

  const headers: Record<string, string> = {
    ...(options.headers as Record<string, string>),
    Authorization: `Bearer ${accessToken}`,
  };

  const response = await fetch(url, {
    ...options,
    headers,
  });

  if (response.status === 401) {
    console.warn("Received 401 Unauthorized, user needs to re-authenticate");
  }

  return response;
}
