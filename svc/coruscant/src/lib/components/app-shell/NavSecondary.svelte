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

const pages = [
    { title: "Docs", url: "/docs", icon: "fa6-regular:file-lines" },
    { title: "Settings", url: "/settings", icon: "fa6-solid:gear" },
    { title: "Roadmap", url: "/roadmap", icon: "fa6-solid:road" },
];
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu>
      {#each pages as item}
        <SidebarMenuItem>
          <SidebarMenuButton href={item.url} isActive={item.url === pathname}>
            <Icon icon={item.icon} class="shrink-0" />
            <span class="group-data-[state=collapsed]:hidden">{item.title}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      {/each}
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
