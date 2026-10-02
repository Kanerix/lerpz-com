<script lang="ts">
import { Badge, Button, ScrollArea } from "@lerpz/ui";
import { createQuery } from "@tanstack/svelte-query";
import { onDestroy } from "svelte";
import { browser } from "$app/environment";
import { readAgent } from "$lib/api/agents/agents.js";
import { msalStore } from "$lib/auth/msal.svelte.js";

import { ErrorState } from "$lib/components/error-state/index.js";
import { queryKeys } from "$lib/query/keys.js";

import { formatDate } from "$lib/utils/format.js";

let { agent }: { agent: string } = $props();
const accountKey = $derived(msalStore.accountKey);
const enabled = $derived(browser && accountKey !== null);
const query = createQuery(() => ({
    queryKey: queryKeys.agents.detail(accountKey, agent),
    enabled: enabled && Boolean(agent),
    queryFn: async ({ signal }) => {
        if (!/^[a-z0-9](?:[a-z0-9-]{0,38}[a-z0-9])?$/.test(agent)) {
            throw new Error("Invalid agent name.");
        }
        const response = await readAgent(agent, { signal });
        if (response.status !== 200) throw response.data;
        return response.data;
    },
    staleTime: 0,
    retry: false,
    refetchInterval: 10_000,
    meta: { skipGlobalErrorDialog: true },
}));
const runtime = $derived(query.data);

let disposed = false;
onDestroy(() => {
    disposed = true;
});

async function refresh() {
    if (disposed || !enabled || query.isFetching) return;
    await query.refetch();
}
</script>

<ScrollArea class="h-full" orientation="vertical">
  <div class="mx-auto flex w-full max-w-5xl flex-col gap-8 px-4 py-8">
    <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
      <div class="flex min-w-0 flex-col items-start gap-3">
        <Button href="/ai/agents/sessions" variant="ghost" size="sm">Back to agents</Button>
        <h1 class="break-all">{runtime?.name || agent}</h1>
        <p class="text-sm text-muted-foreground">Your private agent.</p>
      </div>
      <Button variant="outline" disabled={!enabled || query.isFetching} onclick={refresh}>
        {query.isFetching ? "Refreshing…" : "Refresh"}
      </Button>
    </header>

    {#if !enabled}
      <p role="status" class="text-sm text-muted-foreground">Sign in to view this agent.</p>
    {:else}
      {#if query.isError}
        <div role="alert">
          <ErrorState
            compact
            title={runtime ? "Couldn't refresh this agent" : "Couldn't load this agent"}
            description={runtime ? "The last fetched data is shown below and may be out of date." : "It may have been deleted, or the service may be unavailable."}
            onRetry={refresh}
            retrying={query.isFetching}
            retryLabel="Refresh"
          />
        </div>
      {/if}
      {#if query.isPending}
        <p role="status" class="text-sm text-muted-foreground">Loading agent…</p>
      {:else if runtime}
      <section class="flex flex-col gap-4 border-t border-border pt-6">
        <div class="flex flex-wrap items-center gap-3">
          <h2>Agent</h2>
          <Badge variant="outline" class="capitalize">{runtime.status.replaceAll("_", " ")}</Badge>
        </div>
        <dl class="grid gap-4 text-sm sm:grid-cols-2">
          <div class="flex min-w-0 flex-col gap-1">
            <dt class="text-muted-foreground">Memory</dt>
            <dd>{runtime.memory ? "Enabled" : "Disabled"}</dd>
          </div>
          <div class="flex min-w-0 flex-col gap-1">
            <dt class="text-muted-foreground">Created</dt>
            <dd>{runtime.created_at ? formatDate(runtime.created_at) : "Not reported"}</dd>
          </div>

        </dl>
      </section>

      {/if}
    {/if}
  </div>
</ScrollArea>
