"use client";

import { Button } from "@lerpz/ui/components/button";
import { useRouter } from "next/navigation";
import { signOut } from "@/lib/auth-client";

export default function LogoutButton() {
  const router = useRouter();

  const handleLogout = async () => {
    await signOut({
      fetchOptions: {
        onSuccess: () => {
          router.push("/login");
        },
      },
    });
  };

  return (
    <Button variant="destructive" onClick={handleLogout}>
      logout
    </Button>
  );
}
