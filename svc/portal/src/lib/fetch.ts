"use client";

import { getAccessToken } from "@/lib/msal-auth";

export async function authenticatedFetch(
  url: string,
  options: RequestInit = {},
): Promise<Response> {
  const accessToken = await getAccessToken();

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
