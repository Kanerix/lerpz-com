<script lang="ts">
import Icon from "@iconify/svelte";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { cubicOut } from "svelte/easing";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import {
    deleteChat,
    getListChatsUrl,
    listChats,
} from "$lib/api/chats/chats.js";
import type { Conversation } from "$lib/api/models/index.js";
import { fade, fly } from "$lib/utils/transitions.js";
import ChatInfoDialog from "./ChatInfoDialog.svelte";

const pathname = $derived(page.url.pathname);

const queryClient = useQueryClient();

const query = createQuery(() => ({
    queryKey: [getListChatsUrl()],
    queryFn: ({ signal }: { signal: AbortSignal }) => listChats({ signal }),
}));

// The conversation shown in the info dialog, and whether it is open.
let infoConversation = $state<Conversation | null>(null);
let infoOpen = $state(false);

// Conversations currently being deleted, so the row can show a spinner and
// ignore repeat clicks.
let pendingIds = $state<string[]>([]);

function openInfo(conv: Conversation) {
    infoConversation = conv;
    infoOpen = true;
}

async function handleDelete(conv: Conversation) {
    if (pendingIds.includes(conv.id)) return;
    pendingIds = [...pendingIds, conv.id];
    try {
        await deleteChat(conv.id);
        await queryClient.invalidateQueries({
            queryKey: [getListChatsUrl()],
        });
        // If the deleted chat is the one being viewed, leave it behind.
        if (pathname === `/ai/chats/${conv.id}`) {
            await goto("/ai/chats");
        }
        toast.success("Chat deleted");
    } catch (err) {
        toast.error("Couldn't delete chat", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    } finally {
        pendingIds = pendingIds.filter((id) => id !== conv.id);
    }
}

const MAX_CHATS = 10;

type DateGroup = "Today" | "Yesterday" | "A week ago" | "A month ago";
const DATE_GROUP_ORDER: DateGroup[] = [
    "Today",
    "Yesterday",
    "A week ago",
    "A month ago",
];

function getTime(conv: Conversation): number {
    const dateStr = conv.updated_at ?? conv.created_at;
    return dateStr ? new Date(dateStr).getTime() : 0;
}

function getDateGroup(dateStr: string | null | undefined): DateGroup {
    if (!dateStr) return "A month ago";
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
    if (date >= startOfWeek) return "A week ago";
    return "A month ago";
}

const groups = $derived.by(() => {
    if (query.data?.status !== 200) return [];
    const convs = [...(query.data.data ?? [])]
        .filter((conv) => !conv.archived)
        .sort((a, b) => getTime(b) - getTime(a))
        .slice(0, MAX_CHATS);
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
  <div out:fade={{ duration: 150 }}>
    <SidebarGroup class="group-data-[state=collapsed]:hidden">
      <SidebarGroupLabel><Skeleton class="h-3 w-12" /></SidebarGroupLabel>
      <SidebarGroupContent class="px-2">
        <div class="space-y-1">
          {#each [0.88, 0.76, 0.64, 0.52, 0.40, 0.28] as opacity}
            <Skeleton class="h-8 w-full rounded-md" style="opacity: {opacity}" />
          {/each}
        </div>
      </SidebarGroupContent>
    </SidebarGroup>
  </div>
{:else if query.data?.status !== 200}
  <p class="text-muted-foreground group-data-[state=collapsed]:hidden px-2 py-4 text-center text-xs">Error: {query.data?.status}</p>
{:else if (query.data.data?.length ?? 0) === 0}
  <SidebarGroup class="group-data-[state=collapsed]:hidden">
    <SidebarGroupLabel>Chats</SidebarGroupLabel>
    <SidebarGroupContent>
      <p class="text-muted-foreground px-2 py-4 text-center text-xs">
        No conversations yet.<br />Start a chat to get going.
      </p>
    </SidebarGroupContent>
  </SidebarGroup>
{:else}
  {#each groups as [label, items], gi}
    <SidebarGroup class="group-data-[state=collapsed]:hidden">
      <SidebarGroupLabel>{label}</SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu>
          {#each items as conv, ii (conv.id)}
            {@const href = `/ai/chats/${conv.id}`}
            {@const isPending = pendingIds.includes(conv.id)}
            <SidebarMenuItem>
              <div
                in:fly={{
                  y: 6,
                  duration: 260,
                  delay: gi * 70 + ii * 35,
                  easing: cubicOut,
                }}
              >
                <SidebarMenuButton {href} isActive={pathname === href}>
                  <span class="min-w-0 truncate">{conv.title ?? "Untitled"}</span>
                </SidebarMenuButton>
              </div>
              <DropdownMenu side="right" align="start" sideOffset={4}>
                <DropdownMenuTrigger
                  data-sidebar="menu-action"
                  aria-label="Chat options"
                  class="absolute right-1 top-1/2 flex size-6 -translate-y-1/2 items-center justify-center rounded-md text-muted-foreground opacity-0 outline-none transition-opacity hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:opacity-100 group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100"
                >
                  {#if isPending}
                    <Icon icon="fa6-solid:spinner" class="size-3.5 animate-spin" />
                  {:else}
                    <Icon icon="fa6-solid:ellipsis" class="size-3.5" />
                  {/if}
                </DropdownMenuTrigger>
                <DropdownMenuContent class="w-40">
                  <DropdownMenuItem value="info" onclick={() => openInfo(conv)}>
                    <Icon icon="fa6-solid:circle-info" />
                    Info
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    value="delete"
                    disabled={isPending}
                    onclick={() => handleDelete(conv)}
                    class="text-destructive focus:text-destructive hover:text-destructive"
                  >
                    <Icon icon="fa6-solid:trash" />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </SidebarMenuItem>
          {/each}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  {/each}
{/if}

<ChatInfoDialog bind:open={infoOpen} conversation={infoConversation} />
