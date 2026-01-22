import { SidebarInset, SidebarProvider } from "@lerpz/ui/components/sidebar";
import type { ReactNode } from "react";
import { AuthGuard } from "@/components/auth-guard";
import { Chatbox } from "@/components/chatbox";
import { ChatboxProvider } from "@/components/chatbox/provider";
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
          </SidebarInset>
        </SidebarProvider>
      </ChatboxProvider>
    </AuthGuard>
  );
}
