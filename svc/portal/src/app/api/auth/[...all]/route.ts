import { toNextJsHandler } from "better-auth/next-js";
import { auth } from "@/lib/msal";

export const { POST, GET } = toNextJsHandler(auth);
