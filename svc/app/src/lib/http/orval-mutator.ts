import { getAccessToken } from "$lib/auth/msal-auth.js";
import { publicEnv } from "$lib/env.js";

const API_BASE_URL = publicEnv.PUBLIC_API_URL;

export type ErrorType<TError> = TError & { status: number };

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

    const text = await response.text();
    const data = text ? JSON.parse(text) : undefined;

    return {
        data,
        status: response.status,
        headers: response.headers,
    } as TResponse;
}
