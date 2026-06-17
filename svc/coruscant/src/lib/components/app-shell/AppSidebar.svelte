<script lang="ts">
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Separator as SeparatorComp } from "@lerpz/ui/components/separator";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarRail,
} from "@lerpz/ui/components/sidebar";
import NavModes from "./NavModes.svelte";
import NavSecondary from "./NavSecondary.svelte";
import NewChatButton from "./NewChatButton.svelte";
import SidebarBrand from "./SidebarBrand.svelte";
import SidebarChats from "./SidebarChats.svelte";
import UserInfo from "./UserInfo.svelte";
</script>

<Sidebar collapsible="icon">
  <SidebarHeader>
    <SidebarBrand />
  </SidebarHeader>
  <SeparatorComp />
  <SidebarContent class="overflow-hidden">
    <NavModes />
    <ScrollArea class="min-h-0 flex-1" orientation="vertical">
      <SidebarChats />
    </ScrollArea>
    <NewChatButton />
    <NavSecondary class="mt-auto" />
  </SidebarContent>
  <SeparatorComp />
  <SidebarFooter>
    <UserInfo />
  </SidebarFooter>
  <SidebarRail />
</Sidebar>
