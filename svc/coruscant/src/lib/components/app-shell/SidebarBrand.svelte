<script lang="ts">
import {
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
</script>

<SidebarMenu>
  <SidebarMenuItem>
    <SidebarMenuButton
      size="lg"
      href="/ai/chats"
      class="justify-center group-data-[state=collapsed]:size-8 group-data-[state=collapsed]:p-0"
    >
      <img src="/lerpz.svg" alt="Lerpz" class="size-10 shrink-0 p-2" />
      <span class="text-base font-semibold group-data-[state=collapsed]:hidden">Lerpz AI</span>
    </SidebarMenuButton>
  </SidebarMenuItem>
</SidebarMenu>
