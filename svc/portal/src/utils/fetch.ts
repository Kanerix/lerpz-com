import { getAccessToken } from "@/lib/msal";

export async function createAuthHeaders(
  additionalHeaders: Record<string, string> = {},
): Promise<Record<string, string>> {
  const accessToken = await getAccessToken();

  if (!accessToken) {
    throw new Error("No access token found - user not authenticated");
  }

  const headers: Record<string, string> = {
    ...additionalHeaders,
  };

  headers.Authorization = `Bearer ${accessToken}`;

  return headers;
}

export async function authenticatedFetch(
  url: string,
  options: RequestInit = {},
): Promise<Response> {
  try {
    const headers = await createAuthHeaders(
      (options.headers as Record<string, string>) || {},
    );

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      console.warn("Received 401 Unauthorized, user needs to re-authenticate");
    }

    return response;
  } catch (error) {
    console.error("Error in authenticated fetch:", error);
    throw error;
  }
}
