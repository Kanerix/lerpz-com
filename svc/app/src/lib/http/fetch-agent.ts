import { publicEnv } from "$lib/env.js";
import { fetchWithToken } from "./fetch-api.js";

/** Send within a trusted agent URL without assuming a runtime endpoint schema. */
export async function agentFetch(
    baseUrl: string,
    path: string,
    options: RequestInit = {},
): Promise<Response> {
    const runtimeOrigin = publicEnv.PUBLIC_AGENT_RUNTIME_ORIGIN;
    if (!runtimeOrigin) {
        throw new Error("Agent runtime access is not configured.");
    }

    const base = new URL(baseUrl);
    const isTrustedOrigin = base.origin === new URL(runtimeOrigin).origin;
    const hasCredentials = Boolean(base.username || base.password);
    const hasQueryOrFragment = Boolean(base.search || base.hash);
    const hasApplicationPath =
        base.pathname !== "/" && !base.pathname.endsWith("/");
    if (
        !isTrustedOrigin ||
        hasCredentials ||
        hasQueryOrFragment ||
        !hasApplicationPath
    ) {
        throw new Error(
            "The agent URL must use the trusted runtime origin and an application path.",
        );
    }

    const pathname = path.split("?")[0] ?? "";
    const isRelativePath = path.startsWith("/") && !path.startsWith("//");
    const hasInvalidCharacters =
        path.includes("\\") ||
        path.includes("#") ||
        Array.from(path).some(
            (character) =>
                character.charCodeAt(0) <= 32 ||
                character.charCodeAt(0) === 127,
        );
    const hasEncodedTraversal = /%(?:2e|2f|5c|25)/i.test(pathname);
    if (!isRelativePath || hasInvalidCharacters || hasEncodedTraversal) {
        throw new Error(
            "Use an endpoint path without traversal or encoded separators.",
        );
    }

    const target = new URL(`${base.href}${path}`);
    const staysWithinAgent =
        target.origin === base.origin &&
        target.pathname.startsWith(`${base.pathname}/`);
    if (!staysWithinAgent) {
        throw new Error("The endpoint must stay within this agent's URL.");
    }

    return fetchWithToken(target.href, options);
}
