import type { ReactNode } from "react";
import { AppShell } from "@/components/app-shell";
import { AuthGuard } from "@/components/auth-guard";
import { Chatbox, ChatboxProvider } from "@/components/chatbox";

interface LayoutProps {
  children: ReactNode;
}

export default function ProtectedLayout({ children }: LayoutProps) {
  return (
    <AuthGuard>
      <ChatboxProvider>
        <AppShell>
          <main className="w-full h-full p-4">
            {children}
            <Chatbox />
          </main>
        </AppShell>
      </ChatboxProvider>
    </AuthGuard>
  );
}
