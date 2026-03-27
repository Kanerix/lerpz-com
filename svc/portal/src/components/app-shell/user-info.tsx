"use client";

import { InteractionStatus } from "@azure/msal-browser";
import { useMsal } from "@azure/msal-react";
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
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@lerpz/ui/components/sidebar";
import {
  MdVerified,
  MdNotifications,
  MdUnfoldMore,
  MdLogout,
  MdAutoAwesome,
  MdGroup,
} from "react-icons/md";
import { useEffect, useState } from "react";
import { loginRequest } from "@/lib/msal-config";
import { getCurrentUserPhotoUrl } from "@/services/graph/photo";

export default function SidebarUserInfo() {
  const [avatarSrc, setAvatarSrc] = useState<string>();

  const { instance, inProgress } = useMsal();
  const account = instance.getActiveAccount();
  const accountId = account?.homeAccountId;

  const { isMobile } = useSidebar();

  useEffect(() => {
    let isMounted = true;

    (async () => {
      if (!accountId) {
        setAvatarSrc(undefined);
        return;
      }

      if (inProgress !== InteractionStatus.None) {
        return;
      }

      const url = await getCurrentUserPhotoUrl();
      if (!isMounted) return;

      setAvatarSrc(url ?? undefined);
    })();

    return () => {
      isMounted = false;
    };
  }, [accountId, inProgress]);

  const avatarFallback =
    account?.name
      ?.split(" ")
      .map((n) => (n ? n[0]?.toUpperCase() : ""))
      .filter((n) => n)
      .slice(0, 2)
      .join("") || "?";

  const handleAccountSwitch = async () => {
    try {
      await instance.loginRedirect({
        prompt: "select_account",
        ...loginRequest,
      });
    } catch (error) {
      console.error("Account switch failed:", error);
    }
  };

  const handleLogout = async () => {
    try {
      await instance.logoutRedirect();
    } catch (error) {
      console.error("Logout failed:", error);
    }
  };
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
                <Avatar className="h-8 w-8 rounded-lg">
                  <AvatarImage src={avatarSrc} />
                  <AvatarFallback className="rounded-lg">
                    {avatarFallback}
                  </AvatarFallback>
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">{account?.name}</span>
                  <span className="truncate text-xs">{account?.username}</span>
                </div>
                <MdUnfoldMore className="ml-auto size-4" />
              </SidebarMenuButton>
            }
          ></DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuGroup>
              <DropdownMenuLabel className="p-0 font-normal">
                <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                  <Avatar className="h-8 w-8 rounded-lg">
                    <AvatarImage src={avatarSrc} />
                    <AvatarFallback className="rounded-lg">
                      {avatarFallback}
                    </AvatarFallback>
                  </Avatar>
                  <div className="grid flex-1 text-left text-sm leading-tight">
                    <span className="truncate font-medium">
                      {account?.name}
                    </span>
                    <span className="truncate text-xs">
                      {account?.username}
                    </span>
                  </div>
                </div>
              </DropdownMenuLabel>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              <DropdownMenuItem>
                <MdAutoAwesome />
                Upgrade to Pro
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              <DropdownMenuItem>
                <MdVerified />
                Account
              </DropdownMenuItem>
              <DropdownMenuItem>
                <MdNotifications />
                Notifications
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={handleAccountSwitch}>
              <MdGroup />
              Switch Account
            </DropdownMenuItem>
            <DropdownMenuItem onClick={handleLogout}>
              <MdLogout />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
