import { browser } from "$app/environment";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { getAccessToken } from "$lib/auth/msal-auth.js";
import { publicEnv } from "$lib/env.js";

export type ErrorType<TError> = TError & { status: number };

export function agentAccountKey(): string | null {
    const account = msalStore.activeAccount;
    return account
        ? `${account.homeAccountId}:${account.tenantId}:${account.localAccountId}`
        : null;
}

export function agentsEnabled(): boolean {
    return browser && agentAccountKey() !== null;
}

export async function agentAccessToken(): Promise<string> {
    const account = agentAccountKey();
    if (!browser || !account) {
        throw new Error("Sign in before managing agents.");
    }
    const token = await getAccessToken();
    if (!token || account !== agentAccountKey()) {
        throw new Error(
            "Your account changed or authentication is incomplete. Please try again.",
        );
    }
    return token;
}

export async function agentFetch<TResponse>(
    path: string,
    options: RequestInit,
): Promise<TResponse> {
    const base = new URL(publicEnv.PUBLIC_API_URL);
    const url = new URL(`${base.href.replace(/\/$/, "")}${path}`);
    const resourcePath = path.split("?")[0] ?? "";
    const validPath =
        /^\/api\/v1\/(?:agents|agent-memory)(?:\/[a-z0-9](?:[a-z0-9-]{0,38}[a-z0-9])?)?$/;
    const loopback = ["localhost", "127.0.0.1", "[::1]"].includes(
        base.hostname,
    );
    if (
        (base.protocol !== "https:" &&
            !(base.protocol === "http:" && loopback)) ||
        base.username ||
        base.password ||
        base.search ||
        base.hash ||
        !validPath.test(resourcePath) ||
        url.pathname !== `${base.pathname.replace(/\/$/, "")}${resourcePath}` ||
        url.hash ||
        url.origin !== base.origin
    ) {
        throw new Error(
            "Agent management requires a trusted HTTPS API URL, or HTTP on localhost.",
        );
    }
    const account = agentAccountKey();
    const token = await agentAccessToken();
    const headers = new Headers(options.headers);
    headers.set("Authorization", `Bearer ${token}`);
    const response = await fetch(url, {
        ...options,
        headers,
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        referrerPolicy: "no-referrer",
    });
    if (account !== agentAccountKey()) {
        await response.body?.cancel();
        throw new Error("Your account changed. Please try again.");
    }
    const text = await response.text();
    if (account !== agentAccountKey()) {
        throw new Error("Your account changed. Please try again.");
    }
    if (!response.ok) {
        const error = new Error(
            `HTTP ${response.status}: ${text || response.statusText}`,
        ) as ErrorType<Error>;
        error.status = response.status;
        throw error;
    }
    return {
        data: text ? JSON.parse(text) : undefined,
        status: response.status,
        headers: response.headers,
    } as TResponse;
}
