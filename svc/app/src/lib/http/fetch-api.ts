import { browser } from "$app/environment";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { getAccessToken } from "$lib/auth/msal-auth.js";
import { publicEnv } from "$lib/env.js";

/** Send a bearer-authenticated request to the core API. */
export async function authenticatedFetch(
    url: string,
    options: RequestInit = {},
): Promise<Response> {
    const api = new URL(publicEnv.PUBLIC_API_URL);
    const target = new URL(url, api);

    const loopback = ["localhost", "127.0.0.1", "[::1]"].includes(
        target.hostname,
    );

    const isTrustedOrigin = target.origin === api.origin;
    const isAllowedProtocol =
        target.protocol === "https:" ||
        (target.protocol === "http:" && loopback);
    const hasCredentials = Boolean(target.username || target.password);
    const hasFragment = target.hash !== "";

    if (
        !isTrustedOrigin ||
        !isAllowedProtocol ||
        hasCredentials ||
        hasFragment
    ) {
        throw new Error(
            "Authenticated requests require a trusted HTTPS URL, or HTTP on localhost.",
        );
    }

    return fetchWithToken(target.href, options);
}

/** Send to a destination already validated by the API or runtime fetch wrapper. */
export async function fetchWithToken(
    url: string,
    options: RequestInit = {},
): Promise<Response> {
    const account = msalStore.accountKey;
    if (!browser || !account)
        throw new Error("Sign in before making requests.");

    const accessToken = await getAccessToken();
    if (!accessToken || account !== msalStore.accountKey) {
        throw new Error(
            "Your account changed or authentication is incomplete. Please try again.",
        );
    }

    const headers = new Headers(options.headers);
    headers.set("Authorization", `Bearer ${accessToken}`);
    const response = await fetch(url, {
        ...options,
        headers,
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        referrerPolicy: "no-referrer",
    });
    if (account !== msalStore.accountKey) {
        await response.body?.cancel();
        throw new Error("Your account changed. Please try again.");
    }
    return response;
}
