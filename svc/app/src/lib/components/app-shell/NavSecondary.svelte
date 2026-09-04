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
import { msalStore } from "$lib/auth/msal.svelte.js";

let { class: className = "" }: { class?: string } = $props();

const pathname = $derived(page.url.pathname);

interface NavItem {
    title: string;
    url: string;
    icon: string;
    /** App role required to see this item. Omit to always show it. */
    role?: string;
}

const pages: NavItem[] = [
    { title: "Docs", url: "/docs", icon: "fa6-regular:file-lines" },
    { title: "Settings", url: "/settings", icon: "fa6-solid:gear" },
    {
        title: "Admin",
        url: "/admin",
        icon: "fa6-solid:shield-halved",
        role: "Lerpz.Admin",
    },
];

const visiblePages = $derived(
    pages.filter((item) => !item.role || msalStore.hasRole(item.role)),
);
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu>
      {#each visiblePages as item}
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
