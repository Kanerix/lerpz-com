import type { Configuration, RedirectRequest } from "@azure/msal-browser";
import { env } from "@/lib/env";

export const msalConfig: Configuration = {
  auth: {
    clientId: env.NEXT_PUBLIC_ENTRA_ID_CLIENT_ID,
    authority: env.NEXT_PUBLIC_ENTRA_ID_AUTHORITY,
    redirectUri: env.NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI,
    navigateToLoginRequestUrl: false,
  },
  cache: {
    cacheLocation: "localStorage",
    storeAuthStateInCookie: false,
  },
};

export const loginRequest: RedirectRequest = {
  scopes: [env.NEXT_PUBLIC_ENTRA_ID_SCOPE],
  redirectUri: env.NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI,
};
