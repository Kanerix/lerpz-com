"use client";

import { getAccessToken } from "@/lib/msal-auth";
import { env } from "./env";

const API_BASE_URL = env.NEXT_PUBLIC_API_URL;

export async function authenticatedFetch(
  url: string,
  options: RequestInit = {},
): Promise<Response> {
  const accessToken = await getAccessToken();

  const headers: Record<string, string> = {
    ...(options.headers as Record<string, string>),
    Authorization: `Bearer ${accessToken}`,
  };

  const fullUrl = `${API_BASE_URL}${url}`;

  const response = await fetch(fullUrl, {
    ...options,
    headers,
  });

  if (response.status === 401) {
    console.warn("Received 401 Unauthorized, user needs to re-authenticate");
  }

  return response;
}
