import { getAccessToken } from "$lib/auth/msal-auth.js";

const GRAPH_BASE_URL = "https://graph.microsoft.com/v1.0";

let cachedPhotoUrl: string | null | undefined;

export function clearCurrentUserPhotoCache(): void {
    if (cachedPhotoUrl) {
        URL.revokeObjectURL(cachedPhotoUrl);
    }
    cachedPhotoUrl = undefined;
}

export async function getCurrentUserPhotoBlob(): Promise<Blob | null> {
    const accessToken = await getAccessToken([
        "https://graph.microsoft.com/User.Read",
    ]);
    if (!accessToken) {
        console.warn("[graph/photo] No access token available");
        return null;
    }

    try {
        const response = await fetch(`${GRAPH_BASE_URL}/me/photo/$value`, {
            method: "GET",
            headers: {
                Authorization: `Bearer ${accessToken}`,
            },
        });

        if (response.status === 404) {
            // User has no profile photo configured
            return null;
        }

        if (!response.ok) {
            console.error("[graph/photo] Failed to fetch profile photo", {
                status: response.status,
                statusText: response.statusText,
            });
            return null;
        }

        return await response.blob();
    } catch (error) {
        console.error(
            "[graph/photo] Failed to get photo response as Blob",
            error,
        );
        return null;
    }
}

export async function getCurrentUserPhotoUrl(): Promise<string | null> {
    if (cachedPhotoUrl !== undefined) {
        return cachedPhotoUrl;
    }

    const blob = await getCurrentUserPhotoBlob();
    if (!blob) {
        cachedPhotoUrl = null;
        return null;
    }

    cachedPhotoUrl = URL.createObjectURL(blob);
    return cachedPhotoUrl;
}

export async function getCurrentUserPhotoDataUrl(): Promise<string | null> {
    const blob = await getCurrentUserPhotoBlob();
    if (!blob) {
        return null;
    }

    try {
        const arrayBuffer = await blob.arrayBuffer();
        const bytes = new Uint8Array(arrayBuffer);
        let binary = "";
        for (const b of bytes) {
            binary += String.fromCharCode(b);
        }
        const base64 = btoa(binary);
        const contentType = blob.type || "image/jpeg";

        return `data:${contentType};base64,${base64}`;
    } catch (error) {
        console.error(
            "[graph/photo] Failed to convert Blob to data URL",
            error,
        );
        return null;
    }
}
