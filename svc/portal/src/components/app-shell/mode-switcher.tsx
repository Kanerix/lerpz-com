"use client";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@lerpz/ui/components/sidebar";
import { ChevronsUpDown } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useChatbox } from "@/components/chatbox/provider";
import type { ChatboxVariant } from "@/hooks/useEnhance";

export interface Mode {
  variant: ChatboxVariant;
  name: string;
  logo: React.ElementType;
  plan: string;
  href: string;
  target: string;
}

export interface ModeSwitcherProps {
  defaultMode: Mode;
  modes: Mode[];
}

export default function ModeSwitcher({
  defaultMode,
  modes,
}: ModeSwitcherProps) {
  const router = useRouter();
  const { isMobile } = useSidebar();

  const [activeMode, setActiveMode] = useState<Mode>(defaultMode);

  const { setMode: setVariant } = useChatbox();

  const handleModeChange = (mode: Mode) => {
    setActiveMode(mode);
    setVariant(mode.variant);
    router.push(mode.href);
  };

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <SidebarMenuButton size="lg">
                <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                  <activeMode.logo className="size-4" />
                </div>
                <div className="flex flex-col gap-0.5 leading-none">
                  <span className="truncate font-medium">
                    {activeMode.name}
                  </span>
                  <span className="truncate text-md">{activeMode.plan}</span>
                </div>
                <ChevronsUpDown className="ml-auto" />
              </SidebarMenuButton>
            }
          />
          <DropdownMenuContent
            align="start"
            side={isMobile ? "bottom" : "right"}
            sideOffset={4}
          >
            <DropdownMenuGroup>
              <DropdownMenuLabel className="text-muted-foreground text-xs">
                Modes
              </DropdownMenuLabel>
              {modes.map((mode) => (
                <DropdownMenuItem
                  key={mode.name}
                  onClick={() => handleModeChange(mode)}
                  className="gap-2 p-2"
                >
                  <div className="flex size-6 items-center justify-center rounded-md border">
                    <mode.logo className="size-3.5 shrink-0" />
                  </div>
                  {mode.name}
                </DropdownMenuItem>
              ))}
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
