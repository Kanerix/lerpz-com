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
import { deleteAgent } from "$lib/api/agents/agents.js";
import type { AgentResponse } from "$lib/api/models/index.js";
import { showError } from "$lib/components/error-dialog/index.js";
import { ErrorState } from "$lib/components/error-state/index.js";
import { agentAccountKey, agentsEnabled } from "$lib/http/agent-mutator.js";
import {
    agentStatusLabel,
    agentsQueryKey,
    agentsQueryOptions,
} from "$lib/http/agents.js";
import { formatDate } from "$lib/utils/format.js";

const queryClient = useQueryClient();
const query = createQuery(() => agentsQueryOptions());
const enabled = $derived(agentsEnabled());
const agents = $derived(query.data ?? []);

let pendingDelete = $state<AgentResponse | null>(null);
let deleting = $state(false);
let disposed = false;

onDestroy(() => {
    disposed = true;
});

async function refresh() {
    if (disposed || !enabled || query.isFetching || deleting) return;
    await query.refetch();
}

function cancelDelete() {
    if (!deleting) pendingDelete = null;
}

async function confirmDelete() {
    const agent = pendingDelete;
    const submissionAccount = agentAccountKey();
    if (
        disposed ||
        !agent?.name ||
        !agentsEnabled() ||
        !submissionAccount ||
        deleting ||
        query.isError
    )
        return;

    const queryKey = agentsQueryKey();
    const isCurrentSubmission = () =>
        !disposed && submissionAccount === agentAccountKey();
    deleting = true;
    try {
        const response = await deleteAgent(agent.name);
        if (response.status !== 204) throw response.data;
        if (!isCurrentSubmission()) return;
        pendingDelete = null;
        toast.success("Agent deletion requested", {
            description:
                "Removal may take a moment. Its saved memory is kept for reuse.",
        });
    } catch (err) {
        if (!isCurrentSubmission()) return;
        pendingDelete = null;
        showError(err);
    } finally {
        const refetchType =
            submissionAccount === agentAccountKey() ? "active" : "none";
        await queryClient.invalidateQueries({ queryKey, refetchType });
        if (isCurrentSubmission()) deleting = false;
    }
}
</script>

<ScrollArea class="h-full" orientation="vertical">
  <div class="mx-auto flex w-full max-w-3xl flex-col gap-8 px-4 py-8">
    <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
      <div class="flex flex-col gap-1">
        <h1>Agents</h1>
        <p class="text-sm text-muted-foreground">
          View your agents and connect to them.
        </p>
      </div>
      <Button
        variant="outline"
        disabled={!enabled || query.isFetching || deleting}
        onclick={refresh}
      >
        {query.isFetching ? "Refreshing…" : "Refresh"}
      </Button>
    </header>

    {#if !enabled}
      <p class="text-sm text-muted-foreground" role="status">Sign in to manage your agents.</p>
    {:else}
      {#if query.isError}
        <div role="alert">
          <ErrorState
            compact
            title={query.data ? "Couldn't refresh agents" : "Couldn't load agents"}
            description={query.data ? "The last fetched data is shown below and may be out of date." : "Refresh to try again."}
            onRetry={refresh}
            retrying={query.isFetching || deleting}
            retryLabel="Refresh"
          />
        </div>
      {/if}

      {#if query.isPending}
        <p class="text-sm text-muted-foreground" role="status">Loading agents…</p>
      {:else if query.data && agents.length === 0}
        <p class="border-t border-border py-8 text-sm text-muted-foreground">
          {query.isError ? "The last fetched list contained no agents." : "No agents yet. Create an agent to get started."}
        </p>
      {:else if agents.length > 0}
        <ul class="flex flex-col divide-y divide-border border-y border-border">
          {#each agents as agent (agent.name)}
            <li class="flex min-w-0 flex-col gap-4 py-6">
              <div class="flex flex-wrap items-center gap-2">
                <h2 class="min-w-0 break-all text-sm font-semibold">{agent.name}</h2>
                <Badge variant="outline">{agentStatusLabel(agent.status)}</Badge>
              </div>
              <dl class="grid min-w-0 gap-3 text-sm sm:grid-cols-2">
                <div class="flex min-w-0 flex-col gap-1">
                  <dt class="text-muted-foreground">Memory</dt>
                  <dd>{agent.memory ? "Enabled" : "Disabled"}</dd>
                </div>
                <div class="flex min-w-0 flex-col gap-1">
                  <dt class="text-muted-foreground">Created</dt>
                  <dd>{agent.created_at ? formatDate(agent.created_at) : "Not reported"}</dd>
                </div>
                <div class="flex min-w-0 flex-col gap-1 sm:col-span-2">
                  <dt class="text-muted-foreground">URL</dt>
                  <dd class="break-all font-mono">{agent.url || "Not available"}</dd>
                </div>
              </dl>
              <div class="flex flex-wrap items-center gap-2">
                <Button href={`/ai/agents/sessions/${encodeURIComponent(agent.name)}`} variant="outline" size="sm">
                  View agent
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  disabled={deleting || query.isError}
                  onclick={() => (pendingDelete = agent)}
                >
                  Delete agent
                </Button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
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
        <DialogTitle>Delete agent?</DialogTitle>
        <DialogDescription class="break-words">
          Delete {pendingDelete?.name}? You will no longer be able to connect to this agent.
          Its saved memory is kept and can be reused by an agent with the same name.
        </DialogDescription>
        <div class="flex flex-wrap justify-end gap-2">
          <Button variant="outline" disabled={deleting} onclick={cancelDelete}>Cancel</Button>
          <Button
            variant="destructive"
            disabled={deleting || !enabled || query.isError}
            onclick={confirmDelete}
          >
            {deleting ? "Deleting…" : "Delete agent"}
          </Button>
        </div>
      </div>
    </DialogContent>
  </DialogPositioner>
</Dialog>
