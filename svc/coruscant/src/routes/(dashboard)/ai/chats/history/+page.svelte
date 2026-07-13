<script lang="ts">
import Icon from "@iconify/svelte";
import { Input } from "@lerpz/ui/components/input";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery } from "@tanstack/svelte-query";
import { getListChatsUrl, listChats } from "$lib/api/chats/chats.js";
import type { Conversation } from "$lib/api/models/index.js";
import ChatHistoryTable from "$lib/components/chats/ChatHistoryTable.svelte";

const query = createQuery(() => ({
    queryKey: [getListChatsUrl()],
    queryFn: ({ signal }: { signal: AbortSignal }) => listChats({ signal }),
}));

let search = $state("");
const normalizedSearch = $derived(search.trim().toLowerCase());

function matches(c: Conversation): boolean {
    if (!normalizedSearch) return true;
    const title = (c.title ?? "Untitled").toLowerCase();
    return (
        title.includes(normalizedSearch) ||
        c.model.toLowerCase().includes(normalizedSearch)
    );
}

const allChats = $derived(
    query.data?.status === 200 ? (query.data.data ?? []) : [],
);

const activeChats = $derived(allChats.filter((c) => !c.archived && matches(c)));
const archivedChats = $derived(
    allChats.filter((c) => c.archived && matches(c)),
);
</script>

<div class="mx-auto flex w-full max-w-5xl flex-col gap-10 px-4 py-8">
  <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Chat history</h1>
      <p class="text-sm text-muted-foreground">
        Browse and manage all of your conversations.
      </p>
    </div>
    <div class="relative w-full sm:max-w-xs">
      <Icon
        icon="fa6-solid:magnifying-glass"
        class="pointer-events-none absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground"
      />
      <Input
        value={search}
        oninput={(e) => (search = e.currentTarget.value)}
        placeholder="Search chats…"
        aria-label="Search chats"
        class="pl-9"
      />
    </div>
  </header>

  {#if query.isLoading}
    <div class="flex flex-col gap-2">
      {#each [0, 1, 2, 3, 4, 5] as i (i)}
        <Skeleton class="h-11 w-full rounded-md" style="opacity: {1 - i * 0.13}" />
      {/each}
    </div>
  {:else if query.data?.status !== 200}
    <p class="rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
      Failed to load chats (error {query.data?.status}).
    </p>
  {:else}
    <section class="flex flex-col gap-3">
      <div class="flex items-center gap-2">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          All chats
        </h2>
        <span class="rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">
          {activeChats.length}
        </span>
      </div>
      {#if activeChats.length === 0}
        <p class="rounded-lg border border-dashed border-border px-4 py-10 text-center text-sm text-muted-foreground">
          {normalizedSearch
            ? "No chats match your search."
            : "No conversations yet. Start a chat to see it here."}
        </p>
      {:else}
        <ChatHistoryTable data={activeChats} />
      {/if}
    </section>

    {#if archivedChats.length > 0}
      <section class="flex flex-col gap-3">
        <div class="flex items-center gap-2">
          <Icon icon="fa6-solid:box-archive" class="size-3.5 text-muted-foreground" />
          <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
            Archived
          </h2>
          <span class="rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">
            {archivedChats.length}
          </span>
        </div>
        <ChatHistoryTable data={archivedChats} />
      </section>
    {/if}
  {/if}
</div>
