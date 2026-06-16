<script lang="ts">
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery } from "@tanstack/svelte-query";
import { page } from "$app/state";
import { getListChatsUrl, listChats } from "$lib/api/chats/chats.js";
import type { Conversation } from "$lib/api/models/index.js";

const pathname = $derived(page.url.pathname);

const query = createQuery(() => ({
    queryKey: [getListChatsUrl()],
    queryFn: ({ signal }: { signal: AbortSignal }) => listChats({ signal }),
}));

type DateGroup = "Today" | "Yesterday" | "This week" | "Older";
const DATE_GROUP_ORDER: DateGroup[] = [
    "Today",
    "Yesterday",
    "This week",
    "Older",
];

function getDateGroup(dateStr: string | null | undefined): DateGroup {
    if (!dateStr) return "Older";
    const date = new Date(dateStr);
    const now = new Date();
    const startOfToday = new Date(
        now.getFullYear(),
        now.getMonth(),
        now.getDate(),
    );
    const startOfYesterday = new Date(startOfToday);
    startOfYesterday.setDate(startOfYesterday.getDate() - 1);
    const startOfWeek = new Date(startOfToday);
    startOfWeek.setDate(startOfWeek.getDate() - 7);
    if (date >= startOfToday) return "Today";
    if (date >= startOfYesterday) return "Yesterday";
    if (date >= startOfWeek) return "This week";
    return "Older";
}

const groups = $derived.by(() => {
    if (query.data?.status !== 200) return [];
    const convs = query.data.data ?? [];
    const map = new Map<DateGroup, Conversation[]>();
    for (const conv of convs) {
        const g = getDateGroup(conv.updated_at ?? conv.created_at);
        if (!map.has(g)) map.set(g, []);
        map.get(g)?.push(conv);
    }
    return DATE_GROUP_ORDER.filter((g) => map.has(g)).map(
        (g) => [g, map.get(g) ?? []] as const,
    );
});
</script>

{#if query.isLoading}
  <SidebarGroup>
    <SidebarGroupLabel><Skeleton class="h-3 w-12" /></SidebarGroupLabel>
    <SidebarGroupContent class="px-2">
      <div class="space-y-1">
        {#each [0.88, 0.76, 0.64, 0.52, 0.40, 0.28] as opacity}
          <Skeleton class="h-8 w-full rounded-md" style="opacity: {opacity}" />
        {/each}
      </div>
    </SidebarGroupContent>
  </SidebarGroup>
{:else if query.data?.status !== 200}
  <p class="text-muted-foreground px-2 py-4 text-center text-xs">Error: {query.data?.status}</p>
{:else if (query.data.data?.length ?? 0) === 0}
  <SidebarGroup>
    <SidebarGroupLabel>Chats</SidebarGroupLabel>
    <SidebarGroupContent>
      <p class="text-muted-foreground px-2 py-4 text-center text-xs">
        No conversations yet.<br />Start a chat to get going.
      </p>
    </SidebarGroupContent>
  </SidebarGroup>
{:else}
  {#each groups as [label, items]}
    <SidebarGroup>
      <SidebarGroupLabel>{label}</SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu>
          {#each items as conv}
            {@const href = `/ai/chats/${conv.id}`}
            <SidebarMenuItem>
              <SidebarMenuButton isActive={pathname === href}>
                <a {href} class="flex items-center gap-2">{conv.title ?? "Untitled"}</a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          {/each}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  {/each}
{/if}
