import { SidebarInset, SidebarTrigger } from "@lerpz/ui/components/sidebar";
import { Toaster } from "sonner";
import AppSidebar from "./app-sidebar";

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <AppSidebar>
      <SidebarInset>
        <header className="flex items-center shrink-0 gap-2 border-b px-4 h-12">
          <SidebarTrigger className="ml-1" />
        </header>
        <Toaster />
      </SidebarInset>
      <AppSidebar />
      {children}
    </AppSidebar>
  );
}

export { AppSidebar };
