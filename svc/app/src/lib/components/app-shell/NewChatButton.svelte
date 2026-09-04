<script lang="ts">
import Icon from "@iconify/svelte";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";

let { class: className = "" }: { class?: string } = $props();
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu>
      <SidebarMenuItem>
        <SidebarMenuButton href="/ai/chats" class="font-medium">
          <Icon icon="fa6-solid:plus" class="shrink-0" />
          <span class="group-data-[state=collapsed]:hidden">New chat</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
