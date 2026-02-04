import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  // Used for docker builds, since docker does not load
  // environment variables during build (only at runtime).
  skipValidation: !!process.env.SKIP_ENV_VALIDATION,
  runtimeEnv: {
    NODE_ENV: process.env.NODE_ENV,
    NEXT_PUBLIC_ENTRA_ID_TENANT_ID: process.env.NEXT_PUBLIC_ENTRA_ID_TENANT_ID,
    NEXT_PUBLIC_ENTRA_ID_CLIENT_ID: process.env.NEXT_PUBLIC_ENTRA_ID_CLIENT_ID,
    // NEXT_PUBLIC_ENTRA_ID_SCOPE: process.env.NEXT_PUBLIC_ENTRA_ID_SCOPE,
    NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI: process.env.NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI,
    NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI: process.env.NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI,
  },
  server: {
    NODE_ENV: z.enum(["production", "development", "test"]),
  },
  client: {
    NEXT_PUBLIC_ENTRA_ID_TENANT_ID: z.string(),
    NEXT_PUBLIC_ENTRA_ID_CLIENT_ID: z.string(),
    // NEXT_PUBLIC_ENTRA_ID_SCOPE: z.string(),
    NEXT_PUBLIC_ENTRA_ID_REDIRECT_URI: z.string(),
    NEXT_PUBLIC_ENTRA_ID_LOGOUT_URI: z.string(),
  },
});
