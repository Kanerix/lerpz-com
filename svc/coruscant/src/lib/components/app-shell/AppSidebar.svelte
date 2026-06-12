<script lang="ts">
import Icon from "@iconify/svelte";
import { Separator as SeparatorComp } from "@lerpz/ui/components/separator";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarRail,
} from "@lerpz/ui/components/sidebar";
import ModeSwitcher from "./ModeSwitcher.svelte";
import NavSecondary from "./NavSecondary.svelte";
import SidebarChats from "./SidebarChats.svelte";
import UserInfo from "./UserInfo.svelte";

const modes = [
    {
        variant: "chat" as const,
        name: "Chat",
        icon: "mdi:chat-bubble",
        plan: "Free",
        href: "/ai/chats",
    },
    {
        variant: "image" as const,
        name: "Image",
        icon: "mdi:image",
        plan: "Free",
        href: "/ai/images",
    },
    {
        variant: "video" as const,
        name: "Video",
        icon: "mdi:videocam",
        plan: "Enterprise",
        href: "/ai/videos",
    },
];
</script>

<Sidebar collapsible="icon">
  <SidebarHeader>
    <ModeSwitcher defaultMode={modes[0]!} {modes} />
  </SidebarHeader>
  <SeparatorComp />
  <SidebarContent>
    <SidebarChats />
    <NavSecondary class="mt-auto" />
  </SidebarContent>
  <SeparatorComp />
  <SidebarFooter>
    <UserInfo />
  </SidebarFooter>
  <SidebarRail />
</Sidebar>
