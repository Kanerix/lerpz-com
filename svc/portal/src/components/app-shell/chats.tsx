"use client";

import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useListChats } from "@/services/api/chats/chats";
import type { Conversation } from "@/services/api/models";

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

function groupByDate(
  conversations: Conversation[],
): [DateGroup, Conversation[]][] {
  const groups = new Map<DateGroup, Conversation[]>();

  for (const conv of conversations) {
    const group = getDateGroup(conv.updated_at ?? conv.created_at);
    const existing = groups.get(group);
    if (existing) {
      existing.push(conv);
    } else {
      groups.set(group, [conv]);
    }
  }

  return DATE_GROUP_ORDER.filter((g) => groups.has(g)).map((g) => [
    g,
    groups.get(g) ?? [],
  ]);
}

function ChatsSkeleton() {
  return (
    <SidebarGroup>
      <SidebarGroupLabel>
        <Skeleton className="h-3 w-12" />
      </SidebarGroupLabel>
      <SidebarGroupContent className="px-2">
        <div className="space-y-1">
          {(["sk-1", "sk-2", "sk-3", "sk-4", "sk-5", "sk-6"] as const).map(
            (id, i) => (
              <Skeleton
                key={id}
                className="h-8 w-full rounded-md"
                style={{ opacity: 1 - i * 0.12 }}
              />
            ),
          )}
        </div>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}

export default function SidebarChats() {
  const pathname = usePathname();

  const { data: conversations, isLoading } = useListChats();

  if (isLoading) {
    return <ChatsSkeleton />;
  }

  if (conversations?.status !== 200) {
    return (
      <p className="text-muted-foreground px-2 py-4 text-center text-xs">
        Error: {conversations?.status}
      </p>
    );
  }

  if (!conversations.data.length) {
    return (
      <SidebarGroup>
        <SidebarGroupLabel>Chats</SidebarGroupLabel>
        <SidebarGroupContent>
          <p className="text-muted-foreground px-2 py-4 text-center text-xs">
            No conversations yet.
            <br />
            Start a chat to get going.
          </p>
        </SidebarGroupContent>
      </SidebarGroup>
    );
  }

  const groups = groupByDate(conversations.data);

  return (
    <>
      {groups.map(([label, items]) => (
        <SidebarGroup key={label}>
          <SidebarGroupLabel>{label}</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((conv) => {
                const href = `/ai/chats/${conv.id}`;
                const title = conv.title ?? "Untitled";
                return (
                  <SidebarMenuItem key={conv.id}>
                    <SidebarMenuButton
                      isActive={pathname === href}
                      render={
                        <Link className="flex items-center gap-2" href={href}>
                          {title}
                        </Link>
                      }
                    />
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      ))}
    </>
  );
}
