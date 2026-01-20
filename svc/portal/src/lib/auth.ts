import { betterAuth } from "better-auth";
import { env } from "@/lib/env";

export const auth = betterAuth({
  socialProviders: {
    microsoft: {
      clientId: env.ENTRA_ID_CLIENT_ID,
      clientSecret: env.ENTRA_ID_CLIENT_SECRET,
      redirectURI: env.ENTRA_ID_REDIRECT_URI,
      tenantId: "common",
      authority: "https://login.microsoftonline.com",
      prompt: "select_account",
    },
  },
  session: {
    cookieCache: {
      enabled: true,
      maxAge: 7 * 24 * 60 * 60,
      strategy: "jwe",
      refreshCache: true,
    },
  },
  account: {
    // storeStateStrategy: "cookie",
    storeAccountCookie: false,
  },
});
