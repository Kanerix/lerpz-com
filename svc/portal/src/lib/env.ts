import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  // Used for docker builds, since docker does not load environment variables
  // during build (only at runtime).
  skipValidation: !!process.env.SKIP_ENV_VALIDATION,
  experimental__runtimeEnv: process.env,
  server: {
    NODE_ENV: z.enum(["production", "development" ,"test"]),
    AUTH_SECRET: z.string(),
    AUTH_URL: z.url().optional(),
    AUTH_MICROSOFT_ENTRA_ID_CLIENT_ID: z.uuidv4(),
    AUTH_MICROSOFT_ENTRA_ID_CLIENT_SECRET: z.string(),
    AUTH_MICROSOFT_ENTRA_ID_ISSUER: z.url(),
  },
});
