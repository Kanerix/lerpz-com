// lib/msal.ts
import {
  type Configuration,
  LogLevel,
  PublicClientApplication,
} from "@azure/msal-browser";
import { env } from "./env";

export const msalConfig: Configuration = {
  auth: {
    clientId: env.NEXT_PUBLIC_ENTRA_ID_CLIENT_ID,
    authority: `https://login.microsoftonline.com/${env.NEXT_PUBLIC_ENTRA_ID_TENANT_ID}`,
    redirectUri: env.NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI,
  },
  cache: {
    cacheLocation: "sessionStorage",
  },
  system: {
    loggerOptions: {
      loggerCallback: (level, message, containsPii) => {
        if (containsPii) {
          return;
        }
        switch (level) {
          case LogLevel.Error:
            console.error(message);
            return;
          case LogLevel.Warning:
            console.warn(message);
            return;
        }
      },
    },
  },
};

export const msalInstance = new PublicClientApplication(msalConfig);

export const loginRequest = {
  scopes: [env.NEXT_PUBLIC_ENTRA_ID_SCOPE],
};
