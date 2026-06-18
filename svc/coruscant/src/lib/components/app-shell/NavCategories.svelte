<script lang="ts">
import Icon from "@iconify/svelte";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarMenuSub,
    SidebarMenuSubButton,
    SidebarMenuSubItem,
    useSidebar,
} from "@lerpz/ui/components/sidebar";
import { page } from "$app/state";

let { class: className = "" }: { class?: string } = $props();

const sidebar = useSidebar();
const pathname = $derived(page.url.pathname);

type SubItem = { title: string; href: string; icon: string };
type Category = {
    name: string;
    icon: string;
    href: string;
    items: SubItem[];
};

// Each category groups a media type. Every category leads with a "Create"
// action, then the relevant gallery/tools. Some sub-routes are proposals and
// may not exist yet.
const categories: Category[] = [
    {
        name: "Chat",
        icon: "fa6-regular:message",
        href: "/ai/chats",
        items: [
            { title: "New chat", href: "/ai/chats", icon: "fa6-solid:plus" },
            {
                title: "History",
                href: "/ai/chats/history",
                icon: "fa6-solid:clock-rotate-left",
            },
        ],
    },
    {
        name: "Images",
        icon: "fa6-regular:image",
        href: "/ai/images",
        items: [
            { title: "Generate", href: "/ai/images", icon: "fa6-solid:plus" },
            {
                title: "Gallery",
                href: "/ai/images/gallery",
                icon: "fa6-regular:images",
            },
            {
                title: "Edit",
                href: "/ai/images/edit",
                icon: "fa6-solid:pen-to-square",
            },
            {
                title: "Analysis",
                href: "/ai/images/analysis",
                icon: "fa6-solid:magnifying-glass",
            },
        ],
    },
    {
        name: "Video",
        icon: "fa6-solid:video",
        href: "/ai/videos",
        items: [
            { title: "Generate", href: "/ai/videos", icon: "fa6-solid:plus" },
            {
                title: "Gallery",
                href: "/ai/videos/gallery",
                icon: "fa6-solid:film",
            },
        ],
    },
];

function isCategoryActive(category: Category): boolean {
    return pathname.startsWith(category.href);
}

// Manual overrides win; otherwise a category defaults to open when it's active.
const overrides = $state<Record<string, boolean>>({});

function isOpen(category: Category): boolean {
    return overrides[category.name] ?? isCategoryActive(category);
}

function toggle(category: Category) {
    overrides[category.name] = !isOpen(category);
}
</script>

<SidebarGroup class={className}>
  <SidebarGroupContent>
    <SidebarMenu>
      {#each categories as category (category.name)}
        {@const open = isOpen(category)}
        <SidebarMenuItem>
          {#if sidebar.state === "collapsed"}
            <!-- Collapsed: icon links straight to the category landing page. -->
            <SidebarMenuButton
              href={category.href}
              isActive={isCategoryActive(category)}
              class="justify-center"
              title={category.name}
              aria-label={category.name}
            >
              <Icon icon={category.icon} class="size-4 shrink-0" />
            </SidebarMenuButton>
          {:else}
            <SidebarMenuButton
              isActive={isCategoryActive(category)}
              onclick={() => toggle(category)}
              aria-expanded={open}
            >
              <Icon icon={category.icon} class="size-4 shrink-0" />
              <span class="flex-1">{category.name}</span>
              <Icon
                icon="fa6-solid:angle-right"
                class="size-4 shrink-0 text-sidebar-foreground/70 transition-transform duration-200 {open
                  ? 'rotate-90'
                  : ''}"
              />
            </SidebarMenuButton>
            {#if open}
              <SidebarMenuSub>
                {#each category.items as item (item.href)}
                  <SidebarMenuSubItem>
                    <SidebarMenuSubButton
                      href={item.href}
                      isActive={pathname === item.href}
                    >
                      <Icon icon={item.icon} class="shrink-0" />
                      <span class="truncate">{item.title}</span>
                    </SidebarMenuSubButton>
                  </SidebarMenuSubItem>
                {/each}
              </SidebarMenuSub>
            {/if}
          {/if}
        </SidebarMenuItem>
      {/each}
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
