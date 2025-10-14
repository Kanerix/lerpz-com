import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  experimental__runtimeEnv: process.env,
  server: {
    NODE_ENV: z.enum(["production", "development" ,"test"]),
    AUTH_SECRET: z.string(),
    AUTH_MICROSOFT_ENTRA_ID_CLIENT_ID: z.uuidv4(),
    AUTH_MICROSOFT_ENTRA_ID_CLIENT_SECRET: z.string(),
    AUTH_MICROSOFT_ENTRA_ID_ISSUER: z.url(),
  },
});
