import { msalStore } from "$lib/auth/msal.svelte.js";
import { authenticatedFetch } from "./fetch-api.js";

export type ErrorType<TError> = TError & { status: number };

export async function customFetch<TResponse>(
    url: string,
    options: RequestInit,
): Promise<TResponse> {
    const account = msalStore.accountKey;
    const response = await authenticatedFetch(url, options);
    const text = await response.text();
    if (account !== msalStore.accountKey) {
        throw new Error("Your account changed. Please try again.");
    }

    if (!response.ok) {
        const error = new Error(
            `HTTP ${response.status}: ${text || response.statusText}`,
        ) as ErrorType<Error>;
        error.status = response.status;
        throw error;
    }

    const data = text ? JSON.parse(text) : undefined;

    return {
        data,
        status: response.status,
        headers: response.headers,
    } as TResponse;
}
