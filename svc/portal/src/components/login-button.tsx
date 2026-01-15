"use client";

import { Button } from "@lerpz/ui/components/button";
import { useRouter } from "next/navigation";
import { signIn } from "@/lib/auth-client";

export default function LoginButton() {
  const router = useRouter();

  const handleLogin = async () => {
    await signIn.social({
      provider: "microsoft",
      callbackURL: "/",
      fetchOptions: {
        onSuccess: () => {
          router.push("/login");
        },
      },
    });
  };

  return <Button onClick={handleLogin}>Sign In with Microsoft</Button>;
}
