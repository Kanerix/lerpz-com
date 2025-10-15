import NextAuth from "next-auth";
import MicrosoftEntraID from "next-auth/providers/microsoft-entra-id";
import { env } from "@/lib/env";

export const { handlers, auth, signIn, signOut } = NextAuth({
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
