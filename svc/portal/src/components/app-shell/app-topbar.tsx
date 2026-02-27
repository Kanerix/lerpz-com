import { SidebarTrigger } from "@lerpz/ui/components/sidebar";
import { ThemeButton } from "../theme-button";

export default function AppTopbar() {
  return (
    <header className="flex items-center shrink-0 gap-2 border-b px-4 h-12">
      <div className="grow">
        <SidebarTrigger className="ml-1" />
      </div>
      <div>
        <ThemeButton />
      </div>
    </header>
  );
}
