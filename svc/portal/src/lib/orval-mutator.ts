"use client";

import { getAccessToken } from "@/lib/msal-auth";
import { env } from "./env";

const API_BASE_URL = env.NEXT_PUBLIC_API_URL;

export type ErrorType<TError> = TError & {
  status: number;
};

/**
 * Custom fetch mutator used by all Orval-generated hooks.
 *
 * Orval calls this as: customFetch<TResponse>(url: string, options: RequestInit)
 * The generated code already handles method, Content-Type, body serialization,
 * and forwarding the AbortSignal — this mutator only needs to:
 *  - Acquire a fresh MSAL access token silently (redirects to login if needed)
 *  - Prepend the API base URL to the path
 *  - Inject the Authorization header
 *  - Throw a typed error on non-2xx responses that includes the HTTP status
 */
export async function customFetch<TResponse>(
  url: string,
  options: RequestInit,
): Promise<TResponse> {
  const accessToken = await getAccessToken();

  const fullUrl = `${API_BASE_URL}${url}`;

  const response = await fetch(fullUrl, {
    ...options,
    headers: {
      ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
      ...(options.headers as Record<string, string> | undefined),
    },
  });

  if (!response.ok) {
    let detail: string;
    try {
      detail = await response.text();
    } catch {
      detail = response.statusText;
    }

    const error = new Error(
      `HTTP ${response.status}: ${detail || response.statusText}`,
    ) as ErrorType<Error>;
    error.status = response.status;

    throw error;
  }

  // Handle empty responses (e.g. 204 No Content)
  const text = await response.text();
  const data = text ? JSON.parse(text) : undefined;

  return {
    data,
    status: response.status,
    headers: response.headers,
  } as TResponse;
}
