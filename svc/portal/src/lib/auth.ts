import { env } from "@lerpz/lib/env";
import NextAuth from "next-auth";
import MicrosoftEntraID from "next-auth/providers/microsoft-entra-id";

export const { handlers, auth, signIn, signOut } = NextAuth({
  debug: env.NODE_ENV === "development",
  secret: env.AUTH_SECRET,
  providers: [
    MicrosoftEntraID({
      clientId: env.AUTH_MICROSOFT_ENTRA_ID_CLIENT_ID,
      clientSecret: env.AUTH_MICROSOFT_ENTRA_ID_CLIENT_SECRET,
      issuer: env.AUTH_MICROSOFT_ENTRA_ID_ISSUER,
      authorization: {
        params: {
          prompt: "select_account",
        },
      },
    }),
  ],
});
