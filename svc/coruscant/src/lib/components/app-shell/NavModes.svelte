<script lang="ts">
import Icon from "@iconify/svelte";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { page } from "$app/state";

let { class: className = "" }: { class?: string } = $props();

const pathname = $derived(page.url.pathname);

const modes = [
    { name: "Chat", icon: "mdi:chat-bubble", href: "/ai/chats" },
    { name: "Image", icon: "mdi:image", href: "/ai/images" },
    { name: "Video", icon: "mdi:videocam", href: "/ai/videos" },
];
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu class="flex-row group-data-[state=collapsed]:flex-col">
      {#each modes as mode}
        <SidebarMenuItem class="flex-1">
          <SidebarMenuButton
            href={mode.href}
            isActive={pathname.startsWith(mode.href)}
            class="justify-center"
            title={mode.name}
            aria-label={mode.name}
          >
            <Icon icon={mode.icon} class="shrink-0" />
          </SidebarMenuButton>
        </SidebarMenuItem>
      {/each}
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
