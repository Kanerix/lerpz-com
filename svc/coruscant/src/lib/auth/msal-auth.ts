import { PublicClientApplication } from "@azure/msal-browser";
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

export function getMsalInstance() {
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

    const request = {
        ...loginRequest,
        scopes: scopes ?? loginRequest.scopes,
        account,
    };

    try {
        const response = await msalInstance.acquireTokenSilent(request);
        return response.accessToken;
    } catch {
        await msalInstance.loginRedirect({
            ...loginRequest,
            scopes: scopes ?? loginRequest.scopes,
        });
        return null;
    }
}
