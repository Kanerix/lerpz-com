"use client";

import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@lerpz/ui/components/avatar";
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
import { useEffect, useState } from "react";
import { useChatbox } from "../chatbox/provider";

interface Mode {
  name: string;
  logo: React.ElementType;
  plan: string;
}

interface ModeSwitcherProps {
  modes: Mode[];
}

const fallbackAvatar = {
  chat: "cht",
  image: "img",
  video: "vid",
};

export default function ModeSwitcher({ modes }: ModeSwitcherProps) {
  const router = useRouter();
  const { isMobile } = useSidebar();
  const [activeMode, setActiveMode] = useState(modes[1]);

  const { variant, setVariant } = useChatbox();

  useEffect(() => {
    if (!activeMode) return;

    switch (activeMode.name) {
      case "Video":
        setVariant("video");
        router.push("/video");
        break;
      case "Image":
        setVariant("image");
        router.push("/image");
        break;
      case "Chat":
        setVariant("chat");
        router.push("/chat");
        break;
    }
  }, [activeMode]);

  if (!activeMode) {
    return null;
  }

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <SidebarMenuButton
                size="lg"
                className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                <Avatar size="lg">
                  <AvatarImage alt="@shadcn" />
                  <AvatarFallback className="rounded-md">
                    {fallbackAvatar[variant]}
                  </AvatarFallback>
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight ml-1">
                  <span className="truncate font-semibold text-lg">
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
                  onClick={() => setActiveMode(mode)}
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
