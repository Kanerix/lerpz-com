<script lang="ts">
import { SidebarMenu, SidebarMenuItem } from "@lerpz/ui/components/sidebar";
</script>

<SidebarMenu>
  <SidebarMenuItem>
    <a
      href="/ai/chats"
      class="flex w-full justify-center items-center gap-2 overflow-hidden px-2 py-4 group-data-[state=collapsed]:size-8 group-data-[state=collapsed]:p-0"
    >
      <img src="/lerpz.svg" alt="Lerpz" class="size-8 shrink-0 mr-2" />
      <h1 class="text-4xl font-semibold group-data-[state=collapsed]:hidden">Lerpz AI</h1>
    </a>
  </SidebarMenuItem>
</SidebarMenu>
