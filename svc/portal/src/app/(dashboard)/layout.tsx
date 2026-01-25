import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@lerpz/ui/components/sidebar";
import { Toaster } from "@lerpz/ui/components/sonner";
import type { ReactNode } from "react";
import { AuthGuard } from "@/components/auth-guard";
import { Chatbox, ChatboxProvider } from "@/components/chatbox";
import { AppSidebar } from "@/components/sidebar";

interface LayoutProps {
  children: ReactNode;
}

export default function ProtectedLayout({ children }: LayoutProps) {
  return (
    <AuthGuard>
      <ChatboxProvider>
        <SidebarProvider>
          <AppSidebar />
          <SidebarInset>
            <header className="flex items-center shrink-0 gap-2 border-b px-4 h-12">
              <SidebarTrigger className="ml-1" />
            </header>
            <main className="w-full h-full p-4">
              {children}
              <Chatbox />
            </main>
            <Toaster />
          </SidebarInset>
        </SidebarProvider>
      </ChatboxProvider>
    </AuthGuard>
  );
}
