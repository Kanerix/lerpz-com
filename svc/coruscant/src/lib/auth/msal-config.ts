import { type Configuration, LogLevel } from "@azure/msal-browser";
import {
    PUBLIC_ENTRA_ID_CLIENT_ID,
    PUBLIC_ENTRA_ID_REDIRECT_URI,
    PUBLIC_ENTRA_ID_SCOPE,
    PUBLIC_ENTRA_ID_TENANT_ID,
} from "$env/static/public";

export const msalConfig: Configuration = {
    auth: {
        clientId: PUBLIC_ENTRA_ID_CLIENT_ID,
        authority: `https://login.microsoftonline.com/${PUBLIC_ENTRA_ID_TENANT_ID}`,
        redirectUri: PUBLIC_ENTRA_ID_REDIRECT_URI,
    },
    cache: {
        cacheLocation: "sessionStorage",
    },
    system: {
        loggerOptions: {
            loggerCallback: (level, message, containsPii) => {
                if (containsPii) return;
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

export const loginRequest = {
    scopes: [PUBLIC_ENTRA_ID_SCOPE],
};
