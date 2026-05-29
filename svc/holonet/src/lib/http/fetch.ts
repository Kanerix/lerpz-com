import { getAccessToken } from "$lib/auth/msal-auth.js";
import { PUBLIC_API_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_URL;

export async function authenticatedFetch(
    url: string,
    options: RequestInit = {},
): Promise<Response> {
    const accessToken = await getAccessToken();

    const headers: Record<string, string> = {
        ...(options.headers as Record<string, string>),
        ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
    };

    const fullUrl = `${API_BASE_URL}${url}`;
    const response = await fetch(fullUrl, { ...options, headers });

    if (response.status === 401) {
        console.warn(
            "Received 401 Unauthorized, user needs to re-authenticate",
        );
    }

    return response;
}
