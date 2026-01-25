import { SidebarInset, SidebarProvider } from "@lerpz/ui/components/sidebar";
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
            <main className="w-full h-screen p-4">{children}</main>
            <Chatbox />
            <Toaster />
          </SidebarInset>
        </SidebarProvider>
      </ChatboxProvider>
    </AuthGuard>
  );
}
