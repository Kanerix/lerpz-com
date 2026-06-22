import {
    AuthError,
    InteractionRequiredAuthError,
    PublicClientApplication,
} from "@azure/msal-browser";
import { goto } from "$app/navigation";
import { loginRequest, msalConfig } from "./msal-config.js";

let msalInstance: PublicClientApplication | null = null;

export async function initializeMsal() {
    if (!msalInstance) {
        msalInstance = new PublicClientApplication(msalConfig);
        await msalInstance.initialize();
    }

    try {
        const result = await msalInstance.handleRedirectPromise();
        if (result?.account) {
            msalInstance.setActiveAccount(result.account);
        }
    } catch (e) {
        console.error("MSAL redirect handling failed", e);
    }

    const activeAccount = msalInstance.getActiveAccount();
    if (!activeAccount) {
        const [firstAccount] = msalInstance.getAllAccounts();
        if (firstAccount) {
            msalInstance.setActiveAccount(firstAccount);
        }
    }

    return msalInstance;
}

export async function getAccessToken(
    scopes?: string[],
): Promise<string | null> {
    if (!msalInstance) {
        console.error("MSAL instance not initialized");
        return null;
    }

    const account = msalInstance.getActiveAccount();
    if (!account) {
        console.warn("No accounts found, user needs to sign in");
        return null;
    }

    const requestScopes = scopes ?? loginRequest.scopes;
    const request = {
        ...loginRequest,
        scopes: requestScopes,
        account,
    };

    try {
        const response = await msalInstance.acquireTokenSilent(request);
        return response.accessToken;
    } catch (error) {
        // Only an interaction-required failure should trigger a redirect.
        // Transient errors (network blips, throttling, etc.) must not blow
        // away the user's app state with a full-page redirect.
        if (error instanceof InteractionRequiredAuthError) {
            await msalInstance.acquireTokenRedirect({
                ...loginRequest,
                scopes: requestScopes,
                account,
            });
            return null;
        }

        // Any other failure is unexpected: send the user to the login page,
        // which surfaces the error via the `error` / `error_description`
        // query params.
        console.error("Silent token acquisition failed", error);

        const code =
            error instanceof AuthError
                ? error.errorCode
                : "token_acquisition_failed";
        const description =
            error instanceof Error ? error.message : String(error);

        const params = new URLSearchParams({
            error: code,
            error_description: description,
        });
        await goto(`/login?${params.toString()}`);
        return null;
    }
}
