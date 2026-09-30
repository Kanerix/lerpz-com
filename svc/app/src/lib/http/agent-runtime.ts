import type { AgentResponse } from "$lib/api/models/index.js";
import { publicEnv } from "$lib/env.js";
import { agentAccessToken, agentAccountKey } from "./agent-mutator.js";

/** Validate the recorded URL before attaching a platform bearer token. */
export function runtimeBaseUrl(runtime: AgentResponse): URL {
    if (!publicEnv.PUBLIC_AGENT_RUNTIME_ORIGIN) {
        throw new Error(
            "Set PUBLIC_AGENT_RUNTIME_ORIGIN to enable runtime requests.",
        );
    }
    if (!runtime.url) {
        throw new Error("This agent has no application URL yet.");
    }
    const url = new URL(runtime.url);
    const origin = new URL(publicEnv.PUBLIC_AGENT_RUNTIME_ORIGIN).origin;
    if (
        url.protocol !== "https:" ||
        url.origin !== origin ||
        url.username ||
        url.password ||
        url.search ||
        url.hash ||
        url.pathname === "/" ||
        url.pathname.endsWith("/")
    ) {
        throw new Error(
            "The agent URL must use the trusted runtime origin and an application path.",
        );
    }
    return url;
}

/** Call an application endpoint without allowing redirects or paths outside its runtime. */
export async function runtimeFetch(
    runtime: AgentResponse,
    path: string,
    options: RequestInit = {},
): Promise<Response> {
    const base = runtimeBaseUrl(runtime);
    const pathname = path.split("?")[0] ?? "";
    if (
        !path.startsWith("/") ||
        path.startsWith("//") ||
        path.includes("\\") ||
        path.includes("#") ||
        Array.from(path).some(
            (character) =>
                character.charCodeAt(0) <= 32 ||
                character.charCodeAt(0) === 127,
        ) ||
        /%(?:2e|2f|5c|25)/i.test(pathname)
    ) {
        throw new Error(
            "Use an endpoint path such as /api/health, without traversal or encoded separators.",
        );
    }
    const url = new URL(`${base.href}${path}`);
    if (
        url.origin !== base.origin ||
        !url.pathname.startsWith(`${base.pathname}/`)
    ) {
        throw new Error("The endpoint must stay within this runtime's URL.");
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
    return response;
}
