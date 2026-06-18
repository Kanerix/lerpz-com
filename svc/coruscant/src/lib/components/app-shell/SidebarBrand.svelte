<script lang="ts">
import { SidebarMenu, SidebarMenuItem } from "@lerpz/ui/components/sidebar";
</script>

<SidebarMenu>
  <SidebarMenuItem>
    <a
      href="/ai/chats"
      class="flex w-full items-center gap-2 overflow-hidden p-2 group-data-[state=collapsed]:size-8 group-data-[state=collapsed]:p-0"
    >
      <img src="/lerpz.svg" alt="Lerpz" class="size-12 shrink-0 p-2" />
      <span class="text-2xl font-semibold group-data-[state=collapsed]:hidden">Lerpz AI</span>
    </a>
  </SidebarMenuItem>
</SidebarMenu>
