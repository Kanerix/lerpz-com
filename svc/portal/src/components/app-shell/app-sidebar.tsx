"use client";

import { Separator } from "@lerpz/ui/components/separator";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@lerpz/ui/components/sidebar";
import type { ComponentProps } from "react";
import { MdChatBubble, MdImage, MdVideocam } from "react-icons/md";
import ModeSwitcher, { type ModeSwitcherProps } from "./mode-switcher";
// import { NavMain } from "./nav-main";
// import { NavProjects } from "./nav-projects";
import NavSecondary from "./nav-secondary";
import SidebarUserInfo from "./user-info";

const modes: ModeSwitcherProps["modes"] = [
  {
    variant: "chat",
    name: "Chat",
    logo: MdChatBubble,
    plan: "Free",
    href: "/ai/chats",
    target: "_self",
  },
  {
    variant: "image",
    name: "Image",
    logo: MdImage,
    plan: "Free",
    href: "/ai/images",
    target: "_self",
  },
  {
    variant: "video",
    name: "Video",
    logo: MdVideocam,
    plan: "Enterprise",
    href: "/ai/videos",
    target: "_self",
  },
];

export default function AppSidebar({
  ...props
}: ComponentProps<typeof Sidebar>) {
  const defaultMode = modes[0];

  if (!defaultMode) {
    throw new Error("unreachable");
  }

  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <ModeSwitcher defaultMode={defaultMode} modes={modes} />
      </SidebarHeader>
      <Separator />
      <SidebarContent>
        {/*<NavMain items={data.navMain} />*/}
        {/*<NavProjects projects={data.projects} />*/}
        <NavSecondary className="mt-auto" />
      </SidebarContent>
      <Separator />
      <SidebarFooter>
        <SidebarUserInfo />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
