"use client";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@lerpz/ui/components/sidebar";
import { Image, MessageCircle, Video } from "lucide-react";
import type { ComponentProps } from "react";
// import { NavMain } from "@/components/sidebar/nav-main";
// import { NavProjects } from "@/components/sidebar/nav-projects";
import ModeSwitcher from "./mode-switcher";
import NavSecondary from "./nav-secondary";
import SidebarUserInfo from "./user-info";

const data = {
  user: {
    name: "shadcn",
    email: "m@example.com",
    avatar: "/avatars/shadcn.jpg",
  },
  modes: [
    {
      name: "Chat",
      logo: MessageCircle,
      plan: "Free",
    },
    {
      name: "Image",
      logo: Image,
      plan: "Free",
    },
    {
      name: "Video",
      logo: Video,
      plan: "Enterprise",
    },
  ],
};

export default function AppSidebar({
  ...props
}: ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <ModeSwitcher modes={data.modes} />
      </SidebarHeader>
      <SidebarContent>
        {/*<NavMain items={data.navMain} />*/}
        {/*<NavProjects projects={data.projects} />*/}
        <NavSecondary className="mt-auto" />
      </SidebarContent>
      <SidebarFooter>
        <SidebarUserInfo />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
