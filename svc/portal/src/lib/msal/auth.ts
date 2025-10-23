import { PublicClientApplication } from "@azure/msal-browser";
import { msalConfig } from "./config";

let msalInstance: PublicClientApplication | null = null;

export async function initializeMsal(): Promise<PublicClientApplication> {
  if (!msalInstance) {
    msalInstance = new PublicClientApplication(msalConfig);
    await msalInstance.initialize();
  }
  return msalInstance;
}

export { msalInstance };
