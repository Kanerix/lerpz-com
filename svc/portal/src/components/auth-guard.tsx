"use client";

import { useMsal } from "@azure/msal-react";
import { useRouter } from "next/navigation";
import { type ReactNode, useEffect } from "react";

export function AuthGuard({ children }: { children: ReactNode }) {
  const { accounts } = useMsal();
  const router = useRouter();

  useEffect(() => {
    if (accounts.length === 0) {
      router.replace("/login");
    }
  }, [accounts, router]);

  if (accounts.length === 0) {
    return (
      <main className="flex min-h-screen items-center justify-center">
        <p>Checking authentication…</p>
      </main>
    );
  }

  return <>{children}</>;
}
