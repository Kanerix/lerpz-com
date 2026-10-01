<script lang="ts">
import {
    Badge,
    Button,
    Dialog,
    DialogBackdrop,
    DialogContent,
    DialogDescription,
    DialogPositioner,
    DialogTitle,
    ScrollArea,
} from "@lerpz/ui";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { onDestroy } from "svelte";
import { toast } from "svelte-sonner";
import { browser } from "$app/environment";
import {
    deleteAgentMemory,
    listAgentMemory,
    readAgentMemory,
} from "$lib/api/agents/agents.js";
import type { AgentMemoryResponse } from "$lib/api/models/index.js";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { showError } from "$lib/components/error-dialog/index.js";
import { ErrorState } from "$lib/components/error-state/index.js";
import { formatDate } from "$lib/utils/format.js";

const queryClient = useQueryClient();
const accountKey = $derived(msalStore.accountKey);
const enabled = $derived(browser && accountKey !== null);
const memoryQuery = createQuery(() => ({
    queryKey: ["agents", accountKey, "memory"],
    enabled,
    queryFn: async ({ signal }: { signal: AbortSignal }) => {
        const response = await listAgentMemory({ signal });
        if (response.status !== 200) throw response.data;
        return response.data;
    },
    staleTime: 0,
    retry: false,
    refetchInterval: 10_000,
    meta: { skipGlobalErrorDialog: true },
}));
const memories = $derived(memoryQuery.data ?? []);

let deleting = $state(false);
let pendingDelete = $state<AgentMemoryResponse | null>(null);
let disposed = false;

onDestroy(() => {
    disposed = true;
});

function deletionBlockReason(memory: AgentMemoryResponse): string | null {
    if (!enabled) return "Sign in before deleting memory.";
    if (memoryQuery.isError) return "Refresh the memory list before deleting.";
    if (memoryQuery.isPending || memoryQuery.isFetching) {
        return "Checking memory before allowing deletion.";
    }
    const latest = memories.find(
        (item) => item.agent_name === memory.agent_name,
    );
    if (!latest)
        return "This memory is no longer listed. Refresh before deleting.";
    if (latest.in_use) {
        return "This memory is in use. Delete its agent before deleting memory.";
    }
    return null;
}

async function refresh() {
    if (disposed || !enabled || memoryQuery.isFetching || deleting) return;
    await memoryQuery.refetch();
}

function cancelDelete() {
    if (!deleting) pendingDelete = null;
}

async function confirmDelete() {
    const memory = pendingDelete;
    const submissionAccount = accountKey;
    if (
        disposed ||
        !memory ||
        !enabled ||
        !submissionAccount ||
        deleting ||
        deletionBlockReason(memory)
    )
        return;

    const queryKey = ["agents", submissionAccount];
    const isCurrentSubmission = () =>
        !disposed && submissionAccount === accountKey;
    deleting = true;
    try {
        const latest = await readAgentMemory(memory.agent_name);
        if (!isCurrentSubmission()) return;
        if (latest.status !== 200) throw latest.data;
        if (latest.data.in_use) {
            throw new Error(
                "This memory is now in use. Delete its agent before deleting memory.",
            );
        }
        const response = await deleteAgentMemory(memory.agent_name);
        if (response.status !== 204) throw response.data;
        if (!isCurrentSubmission()) return;
        pendingDelete = null;
        toast.success("Memory deletion requested", {
            description:
                "Removal may take a moment. Its stored data cannot be recovered here.",
        });
    } catch (err) {
        if (!isCurrentSubmission()) return;
        pendingDelete = null;
        showError(err);
    } finally {
        const refetchType =
            submissionAccount === accountKey ? "active" : "none";
        await queryClient.invalidateQueries({ queryKey, refetchType });
        if (isCurrentSubmission()) deleting = false;
    }
}
</script>

<ScrollArea class="h-full" orientation="vertical">
  <div class="mx-auto flex w-full max-w-3xl flex-col gap-8 px-4 py-8">
    <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
      <div class="flex flex-col gap-1">
        <h1>Memory</h1>
        <p class="text-sm text-muted-foreground">
          Saved memory for your agents. Deleting an agent keeps its memory.
        </p>
      </div>
      <Button
        variant="outline"
        disabled={!enabled || memoryQuery.isFetching || deleting}
        onclick={refresh}
      >
        {memoryQuery.isFetching ? "Refreshing…" : "Refresh"}
      </Button>
    </header>

    {#if !enabled}
      <p class="text-sm text-muted-foreground" role="status">Sign in to manage agent memory.</p>
    {:else}
      <section class="flex flex-col gap-4" aria-labelledby="memory-heading">
        <h2 id="memory-heading">Agent memory</h2>
        {#if memoryQuery.isError}
          <div role="alert">
            <ErrorState
              compact
              title={memoryQuery.data ? "Couldn't refresh agent memory" : "Couldn't load agent memory"}
              description={memoryQuery.data ? "The last fetched data is shown below and may be out of date. Refresh before deleting memory." : "Refresh to try again."}
              onRetry={refresh}
              retrying={memoryQuery.isFetching || deleting}
              retryLabel="Refresh"
            />
          </div>
        {/if}

        {#if memoryQuery.isPending}
          <p class="text-sm text-muted-foreground" role="status">Loading agent memory…</p>
        {:else if memoryQuery.data && memories.length === 0}
          <p class="border-t border-border py-8 text-sm text-muted-foreground">
            {memoryQuery.isError ? "The last fetched list contained no agent memory." : "No saved memory. Enable memory when creating an agent to keep its data."}
          </p>
        {:else if memories.length > 0}
          <ul class="flex flex-col divide-y divide-border border-y border-border">
            {#each memories as memory (memory.agent_name)}
              {@const blockReason = deletionBlockReason(memory)}
              <li class="flex min-w-0 flex-col gap-4 py-6">
                <div class="flex flex-wrap items-center gap-2">
                  <h3 class="min-w-0 break-all text-sm font-semibold">{memory.agent_name}</h3>
                  <Badge variant="outline" class="capitalize">{memory.status.replaceAll("_", " ")}</Badge>
                </div>
                <dl class="grid min-w-0 gap-3 text-sm sm:grid-cols-2">
                  <div class="flex min-w-0 flex-col gap-1">
                    <dt class="text-muted-foreground">Usage</dt>
                    <dd>{memory.in_use ? "In use" : "Not in use"}</dd>
                  </div>
                  <div class="flex min-w-0 flex-col gap-1">
                    <dt class="text-muted-foreground">Created</dt>
                    <dd>{memory.created_at ? formatDate(memory.created_at) : "Not reported"}</dd>
                  </div>
                </dl>
                <div class="flex flex-col items-start gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    disabled={deleting || blockReason !== null}
                    aria-describedby={blockReason ? `delete-block-${memory.agent_name}` : undefined}
                    onclick={() => (pendingDelete = memory)}
                  >
                    Delete memory
                  </Button>
                  {#if blockReason}
                    <p id={`delete-block-${memory.agent_name}`} class="break-words text-xs text-muted-foreground">{blockReason}</p>
                  {/if}
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {/if}
  </div>
</ScrollArea>

<Dialog
  role="alertdialog"
  open={pendingDelete !== null}
  closeOnEscape={!deleting}
  closeOnInteractOutside={!deleting}
  onOpenChange={(details: { open: boolean }) => { if (!details.open) cancelDelete(); }}
>
  <DialogBackdrop />
  <DialogPositioner>
    <DialogContent class="max-h-[85vh] w-full max-w-md overflow-y-auto">
      <div class="flex flex-col gap-4 p-6">
        <DialogTitle>Delete agent memory?</DialogTitle>
        <DialogDescription class="break-words">
          Delete saved memory for {pendingDelete?.agent_name}? This can permanently destroy its stored data.
          Deletion cannot be undone here.
        </DialogDescription>
        <p class="text-sm text-muted-foreground">
          We will check that the memory is not in use before requesting deletion.
        </p>
        {#if pendingDelete && !deleting}
          {@const blockReason = deletionBlockReason(pendingDelete)}
          {#if blockReason}
            <p class="break-words text-sm text-destructive" role="status">{blockReason}</p>
          {/if}
        {/if}
        <div class="flex flex-wrap justify-end gap-2">
          <Button variant="outline" disabled={deleting} onclick={cancelDelete}>Cancel</Button>
          <Button
            variant="destructive"
            disabled={deleting || !pendingDelete || deletionBlockReason(pendingDelete) !== null}
            onclick={confirmDelete}
          >
            {deleting ? "Checking and deleting…" : "Delete memory"}
          </Button>
        </div>
      </div>
    </DialogContent>
  </DialogPositioner>
</Dialog>
