import { SidebarInset, SidebarProvider } from "@lerpz/ui/components/sidebar";
import type { ReactNode } from "react";
import { AuthGuard } from "@/components/auth-guard";
import { AppSidebar } from "@/components/sidebar";

interface LayoutProps {
  children: ReactNode;
}

export default function ProtectedLayout({ children }: LayoutProps) {
  return (
    <AuthGuard>
      <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
          <main>{children}</main>
        </SidebarInset>
      </SidebarProvider>
    </AuthGuard>
  );
}
