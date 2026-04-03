"use client";

import { getAccessToken } from "@/lib/msal-auth";

// The base URL for the portal API. Falls back to localhost for local dev.
const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3001/api/v1";

/**
 * The shape Orval passes to the mutator when using `client: "react-query"`.
 */
export type MutatorArgs = {
  url: string;
  method: string;
  headers?: Record<string, string>;
  params?: Record<string, string | number | boolean>;
  data?: unknown;
  signal?: AbortSignal;
};

/**
 * Error shape thrown by the mutator on non-2xx responses.
 * Orval uses `ErrorType<E>` in its generated hook typings.
 */
export type ErrorType<TError> = TError & {
  status: number;
};

/**
 * Custom fetch mutator used by all Orval-generated hooks.
 *
 * Responsibilities:
 *  - Acquires a fresh MSAL access token silently (redirects to login if needed)
 *  - Builds the full URL from the API base + path + query params
 *  - Forwards the AbortSignal so TanStack Query can cancel in-flight requests
 *  - Throws a typed error on non-2xx responses that includes the HTTP status
 */
export async function customFetch<TResponse>(
  args: MutatorArgs,
): Promise<TResponse> {
  const { url, method, headers, params, data, signal } = args;

  const accessToken = await getAccessToken();

  const queryString =
    params && Object.keys(params).length > 0
      ? `?${new URLSearchParams(
          Object.entries(params).reduce<Record<string, string>>(
            (acc, [k, v]) => {
              acc[k] = String(v);
              return acc;
            },
            {},
          ),
        ).toString()}`
      : "";

  const fullUrl = `${API_BASE_URL}${url}${queryString}`;

  const response = await fetch(fullUrl, {
    method,
    headers: {
      "Content-Type": "application/json",
      ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
      ...headers,
    },
    ...(data !== undefined ? { body: JSON.stringify(data) } : {}),
    signal,
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
  if (!text) {
    return undefined as TResponse;
  }

  return JSON.parse(text) as TResponse;
}