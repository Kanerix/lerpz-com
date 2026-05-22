import { SidebarInset, SidebarProvider } from "@lerpz/ui/components/sidebar";
import AppSidebar from "./app-sidebar";
import AppTopbar from "./app-topbar";

export function AppShell({ children }: { children: React.ReactNode }) {
    return (
        <SidebarProvider>
            <AppSidebar />
            <SidebarInset>
                <AppTopbar />
                {children}
            </SidebarInset>
        </SidebarProvider>
    );
}

export { AppSidebar };
