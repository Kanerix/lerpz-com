"use client";

import type { ReactNode } from "react";
import { AppShell } from "@/components/app-shell";
import { AuthGuard } from "@/components/auth-guard";
import { ChatboxProvider, type ChatboxSubmitArgs } from "@/components/chatbox";

export default function ProtectedLayout({ children }: { children: ReactNode }) {
  const handleSubmit = async (args: ChatboxSubmitArgs) => {
    console.log("Chat submit:", args);
  };

  return (
    <AuthGuard>
      <ChatboxProvider onSubmit={handleSubmit}>
        <AppShell>
          <main className="w-full h-full p-4">{children}</main>
        </AppShell>
      </ChatboxProvider>
    </AuthGuard>
  );
}