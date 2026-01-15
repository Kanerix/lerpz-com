import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  // Used for docker builds, since docker does not load
  // environment variables during build (only at runtime).
  skipValidation: !!process.env.SKIP_ENV_VALIDATION,
  runtimeEnv: {
    NODE_ENV: process.env.NODE_ENV,
    ENTRA_ID_CLIENT_ID: process.env.ENTRA_ID_CLIENT_ID,
    ENTRA_ID_CLIENT_SECRET: process.env.ENTRA_ID_CLIENT_SECRET,
  },
  server: {
    NODE_ENV: z.enum(["production", "development", "test"]),
    ENTRA_ID_CLIENT_ID: z.string(),
    ENTRA_ID_CLIENT_SECRET: z.string(),
  },
});
