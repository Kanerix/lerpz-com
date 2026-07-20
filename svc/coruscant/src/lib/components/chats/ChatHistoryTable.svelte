<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { useQueryClient } from "@tanstack/svelte-query";
import {
    type ColumnDef,
    getCoreRowModel,
    getSortedRowModel,
    type Header,
    type SortingState,
} from "@tanstack/table-core";
import { toast } from "svelte-sonner";
import { updateChat } from "$lib/api/chats/archive.js";
import { getListChatsUrl } from "$lib/api/chats/chats.js";
import type { Conversation } from "$lib/api/models/index.js";
import { createSvelteTable } from "$lib/utils/table.svelte.js";

let {
    data,
}: {
    data: Conversation[];
} = $props();

const queryClient = useQueryClient();

let pendingIds = $state<string[]>([]);

async function toggleArchive(conv: Conversation) {
    if (pendingIds.includes(conv.id)) return;
    const nextArchived = !conv.archived;
    pendingIds = [...pendingIds, conv.id];
    try {
        await updateChat(conv.id, { archived: nextArchived });
        await queryClient.invalidateQueries({
            queryKey: [getListChatsUrl()],
        });
    } catch (err) {
        toast.error(
            nextArchived ? "Couldn't archive chat" : "Couldn't restore chat",
            {
                description:
                    err instanceof Error ? err.message : "Please try again.",
            },
        );
    } finally {
        pendingIds = pendingIds.filter((id) => id !== conv.id);
    }
}

let sorting = $state<SortingState>([{ id: "updated_at", desc: true }]);

const columns: ColumnDef<Conversation>[] = [
    {
        id: "title",
        header: "Title",
        accessorFn: (c) => c.title ?? "Untitled",
    },
    {
        accessorKey: "model",
        header: "Model",
    },
    {
        id: "updated_at",
        header: "Last updated",
        accessorFn: (c) =>
            new Date(c.updated_at ?? c.created_at ?? 0).getTime(),
    },
    {
        id: "created_at",
        header: "Created",
        accessorFn: (c) => new Date(c.created_at ?? 0).getTime(),
    },
    {
        id: "actions",
        header: "",
        enableSorting: false,
    },
];

const table = createSvelteTable({
    get data() {
        return data;
    },
    columns,
    state: {
        get sorting() {
            return sorting;
        },
    },
    onSortingChange: (updater) => {
        sorting = typeof updater === "function" ? updater(sorting) : updater;
    },
    getRowId: (row) => row.id,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
});

function headerText(header: Header<Conversation, unknown>): string {
    const label = header.column.columnDef.header;
    return typeof label === "string" ? label : "";
}

function formatDate(value: string | null | undefined): string {
    if (!value) return "—";
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return "—";
    return date.toLocaleDateString(undefined, {
        year: "numeric",
        month: "short",
        day: "numeric",
    });
}
</script>

<div class="overflow-x-auto rounded-xl border border-border">
  <table class="w-full border-collapse text-sm">
    <thead>
      {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
        <tr class="border-b border-border bg-muted/40">
          {#each headerGroup.headers as header (header.id)}
            <th class="px-3 py-2.5 text-left font-medium text-muted-foreground {header.column.id === 'actions' ? 'text-right' : ''} {header.column.id === 'title' ? 'w-1/4' : ''}">
              {#if header.column.getCanSort()}
                <button
                  type="button"
                  onclick={header.column.getToggleSortingHandler()}
                  class="inline-flex cursor-pointer items-center gap-1.5 transition-colors hover:text-foreground"
                >
                  {headerText(header)}
                  {#if header.column.getIsSorted() === "asc"}
                    <Icon icon="fa6-solid:arrow-up-short-wide" class="size-3" />
                  {:else if header.column.getIsSorted() === "desc"}
                    <Icon icon="fa6-solid:arrow-down-wide-short" class="size-3" />
                  {:else}
                    <Icon icon="fa6-solid:sort" class="size-3 opacity-30" />
                  {/if}
                </button>
              {:else}
                {headerText(header)}
              {/if}
            </th>
          {/each}
        </tr>
      {/each}
    </thead>
    <tbody>
      {#each table.getRowModel().rows as row (row.id)}
        {@const isPending = pendingIds.includes(row.original.id)}
        <tr class="border-b border-border/60 transition-colors last:border-0 hover:bg-muted/40">
          <td class="w-1/4 max-w-0 px-3 py-2.5">
            <a
              href={`/ai/chats/${row.original.id}`}
              class="block truncate font-medium hover:underline"
              title={row.original.title ?? "Untitled"}
            >
              {row.original.title ?? "Untitled"}
            </a>
          </td>
          <td class="px-3 py-2.5 text-muted-foreground">
            <span class="inline-flex items-center rounded-md bg-muted px-2 py-0.5 text-xs">
              {row.original.model}
            </span>
          </td>
          <td class="whitespace-nowrap px-3 py-2.5 text-muted-foreground">
            {formatDate(row.original.updated_at ?? row.original.created_at)}
          </td>
          <td class="whitespace-nowrap px-3 py-2.5 text-muted-foreground">
            {formatDate(row.original.created_at)}
          </td>
          <td class="px-3 py-2.5 text-right">
            <Button
              variant="ghost"
              size="sm"
              disabled={isPending}
              onclick={() => toggleArchive(row.original)}
            >
              {#if isPending}
                <Icon icon="fa6-solid:spinner" class="size-3.5 animate-spin" />
              {:else}
                <Icon
                  icon={row.original.archived ? "fa6-solid:box-open" : "fa6-solid:box-archive"}
                  class="size-3.5"
                />
              {/if}
              {row.original.archived ? "Unarchive" : "Archive"}
            </Button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
