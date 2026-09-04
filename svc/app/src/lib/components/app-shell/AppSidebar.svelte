<script lang="ts">
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { Separator as SeparatorComp } from "@lerpz/ui/components/separator";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarRail,
} from "@lerpz/ui/components/sidebar";
import NavCategories from "./NavCategories.svelte";
import NavSecondary from "./NavSecondary.svelte";
import NewChatButton from "./NewChatButton.svelte";
import SidebarChats from "./SidebarChats.svelte";
import UserInfo from "./UserInfo.svelte";
</script>

<Sidebar collapsible="icon">
  <!-- <SidebarHeader>
    <SidebarBrand />
  </SidebarHeader>
  <SeparatorComp /> -->
  <SidebarContent class="overflow-hidden">
    <NavCategories />
    <NewChatButton />
    <ScrollArea class="min-h-0 shrink" orientation="vertical">
      <SidebarChats />
    </ScrollArea>
    <NavSecondary class="mt-auto" />
  </SidebarContent>
  <SeparatorComp />
  <SidebarFooter>
    <UserInfo />
  </SidebarFooter>
  <SidebarRail />
</Sidebar>
