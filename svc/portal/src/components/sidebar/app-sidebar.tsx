"use client";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@lerpz/ui/components/sidebar";
import { Image, MessageCircle, Video } from "lucide-react";
// import { NavMain } from "@/components/sidebar/nav-main";
// import { NavProjects } from "@/components/sidebar/nav-projects";
import { ModeSwitcher } from "@/components/sidebar/mode-switcher";
import { NavUser } from "./user-info";

// This is sample data.
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
}: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <ModeSwitcher modes={data.modes} />
      </SidebarHeader>
      <SidebarContent>
        {/*<NavMain items={data.navMain} />
        <NavProjects projects={data.projects} />*/}
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={data.user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
