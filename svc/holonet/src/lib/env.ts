import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

const penv = process.env;

export const env = createEnv({
    // Used for docker builds, since docker does not load
    // environment variables during build (only at runtime).
    skipValidation: !!penv.SKIP_ENV_VALIDATION,
    runtimeEnv: {
        NODE_ENV: penv.NODE_ENV,
        NEXT_PUBLIC_API_URL: penv.NEXT_PUBLIC_API_URL,
        NEXT_PUBLIC_ENTRA_ID_TENANT_ID: penv.NEXT_PUBLIC_ENTRA_ID_TENANT_ID,
        NEXT_PUBLIC_ENTRA_ID_CLIENT_ID: penv.NEXT_PUBLIC_ENTRA_ID_CLIENT_ID,
        NEXT_PUBLIC_ENTRA_ID_SCOPE: penv.NEXT_PUBLIC_ENTRA_ID_SCOPE,
        NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI:
            penv.NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI,
        NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI: penv.NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI,
    },
    server: {
        NODE_ENV: z.enum(["production", "development", "test"]),
    },
    client: {
        NEXT_PUBLIC_API_URL: z.url(),
        NEXT_PUBLIC_ENTRA_ID_TENANT_ID: z.string(),
        NEXT_PUBLIC_ENTRA_ID_CLIENT_ID: z.string(),
        NEXT_PUBLIC_ENTRA_ID_SCOPE: z.string(),
        NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI: z.string(),
        NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI: z.string(),
    },
});
