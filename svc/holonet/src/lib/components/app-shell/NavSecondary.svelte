<script lang="ts">
import Icon from "@iconify/svelte";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { page } from "$app/stores";

let { class: className = "" }: { class?: string } = $props();

const pathname = $derived($page.url.pathname);

const pages = [
    { title: "Docs", url: "/docs", icon: "mdi:file-document-outline" },
    { title: "Settings", url: "/settings", icon: "mdi:cog-outline" },
    { title: "Roadmap", url: "/roadmap", icon: "mdi:road-variant" },
];
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu>
      {#each pages as item}
        <SidebarMenuItem>
          <SidebarMenuButton href={item.url} isActive={item.url === pathname}>
            <Icon icon={item.icon} />
            <span>{item.title}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      {/each}
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
