<script lang="ts">
import Icon from "@iconify/svelte";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    useSidebar,
} from "@lerpz/ui/components/sidebar";
import { cubicOut } from "svelte/easing";
import { page } from "$app/state";
import { fly, slide } from "$lib/utils/transitions.js";

let { class: className = "" }: { class?: string } = $props();

const sidebar = useSidebar();
const pathname = $derived(page.url.pathname);

interface SubItem {
    title: string;
    href: string;
    icon: string;
}

interface Category {
    name: string;
    icon: string;
    href: string;
    items: SubItem[];
}

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
    {
        name: "Agents",
        icon: "fa6-solid:robot",
        href: "/ai/agents",
        items: [
            { title: "New agent", href: "/ai/agents", icon: "fa6-solid:plus" },
            {
                title: "Sessions",
                href: "/ai/agents/sessions",
                icon: "fa6-solid:clock-rotate-left",
            },
        ],
    },
];

function isCategoryActive(category: Category): boolean {
    return pathname.startsWith(category.href);
}

const overrides = $state<Record<string, boolean>>({});

function isOpen(category: Category): boolean {
    return overrides[category.name] ?? isCategoryActive(category);
}

function toggle(category: Category) {
    overrides[category.name] = !isOpen(category);
}
</script>

<SidebarGroup class={className}>
  <SidebarGroupLabel>Navigation</SidebarGroupLabel>
  <SidebarGroupContent>
    <SidebarMenu class="gap-1">
      {#each categories as category (category.name)}
        {@const open = isOpen(category)}
        <SidebarMenuItem>
          {#if sidebar.state === "collapsed"}
            <!-- Collapsed: icon links straight to the category landing page. -->
            <SidebarMenuButton
              href={category.href}
              class="justify-center"
              title={category.name}
              aria-label={category.name}
            >
              <Icon icon={category.icon} class="size-4 shrink-0" />
            </SidebarMenuButton>
          {:else}
            <SidebarMenuButton
              onclick={() => toggle(category)}
              aria-expanded={open}
              class="h-10 gap-2.5 rounded-lg px-2.5"
            >
              <span
                class="flex size-7 shrink-0 items-center justify-center
                rounded-md text-sidebar-foreground/60
                transition-[color,transform] duration-200
                group-hover/menu-item:scale-110
                group-hover/menu-item:text-sidebar-foreground"
              >
                <Icon icon={category.icon} class="size-4" />
              </span>
              <span class="flex-1 font-normal">
                  {category.name}
              </span>
              <Icon
                icon="fa6-solid:angle-right"
                class="size-3.5 shrink-0 text-sidebar-foreground/60 transition-transform duration-200
                {open ? 'rotate-90' : ''}"
              />
            </SidebarMenuButton>
            {#if open}
              <ul
                transition:slide={{ duration: 220, easing: cubicOut }}
                class="mt-1 ml-6 flex flex-col gap-1 overflow-hidden border-l border-sidebar-border pl-3"
              >
                {#each category.items as item, i (item.href)}
                  {@const subActive = pathname === item.href}
                  <li in:fly={{ x: -8, duration: 220, delay: 60 + i * 45, easing: cubicOut }}>
                    <a
                      href={item.href}
                      aria-current={subActive ? "page" : undefined}
                      class="flex items-center gap-2.5 rounded-md px-3 py-2 text-sm outline-none transition-colors
                        focus-visible:ring-2 focus-visible:ring-sidebar-ring
                        {subActive
                          ? 'bg-sidebar-accent font-medium text-sidebar-foreground'
                          : 'text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-foreground'}"
                    >
                      <Icon
                          icon={item.icon}
                          class="size-3.5 shrink-0 text-sidebar-foreground/60"
                        />
                      <span class="truncate">{item.title}</span>
                    </a>
                  </li>
                {/each}
              </ul>
            {/if}
          {/if}
        </SidebarMenuItem>
      {/each}
    </SidebarMenu>
  </SidebarGroupContent>
</SidebarGroup>
