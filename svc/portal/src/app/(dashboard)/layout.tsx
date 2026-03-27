"use client";

import { Toaster } from "@lerpz/ui/components/sonner";
import type { ReactNode } from "react";
import { AppShell } from "@/components/app-shell";
import { AuthGuard } from "@/components/auth-guard";
import { ChatboxProvider } from "@/components/chatbox";

export default function ProtectedLayout({ children }: { children: ReactNode }) {
  return (
    <AuthGuard>
      <ChatboxProvider>
        <AppShell>
          <main className="w-full h-full p-4">{children}</main>
          <Toaster />
        </AppShell>
      </ChatboxProvider>
    </AuthGuard>
  );
}
