import { initializeMsal, msalInstance } from "./auth";
import { loginRequest, msalConfig } from "./config";

export { initializeMsal, msalInstance, loginRequest, msalConfig };

export async function getAccessToken(): Promise<string | null> {
  if (!msalInstance) {
    console.error("MSAL instance not initialized");
    return null;
  }

  const accounts = msalInstance.getAllAccounts();
  if (accounts.length === 0) {
    console.warn("No accounts found, user needs to sign in");
    return null;
  }

  const request = {
    ...loginRequest,
    account: accounts[0],
  };

  try {
    const response = await msalInstance.acquireTokenSilent(request);
    return response.accessToken;
  } catch (error) {
    console.error("Failed to acquire token silently:", error);
    return null;
  }
}

export function getCurrentUser() {
  if (!msalInstance) {
    return null;
  }

  const accounts = msalInstance.getAllAccounts();
  if (accounts.length === 0) {
    return null;
  }

  const account = accounts[0];
  if (account === undefined) {
    return null;
  }

  return {
    email: account.username,
    name: account.name,
    id: account.homeAccountId,
  };
}
