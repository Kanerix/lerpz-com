import { type Configuration, LogLevel } from "@azure/msal-browser";
import { publicEnv } from "$lib/env.js";

export const msalConfig: Configuration = {
    auth: {
        clientId: publicEnv.PUBLIC_ENTRA_ID_CLIENT_ID,
        authority: `https://login.microsoftonline.com/${publicEnv.PUBLIC_ENTRA_ID_TENANT_ID}`,
        redirectUri: publicEnv.PUBLIC_ENTRA_ID_REDIRECT_URI,
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
    scopes: [publicEnv.PUBLIC_ENTRA_ID_SCOPE],
};
