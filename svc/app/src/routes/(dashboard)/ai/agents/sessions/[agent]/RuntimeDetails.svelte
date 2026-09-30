<script lang="ts">
import { Badge, Button, Input, ScrollArea } from "@lerpz/ui";
import { createQuery } from "@tanstack/svelte-query";
import { onDestroy } from "svelte";
import { getErrorMessage } from "$lib/components/error-dialog/problem.js";
import { ErrorState } from "$lib/components/error-state/index.js";
import { agentAccountKey, agentsEnabled } from "$lib/http/agent-mutator.js";
import { runtimeBaseUrl, runtimeFetch } from "$lib/http/agent-runtime.js";
import { agentQueryOptions, agentStatusLabel } from "$lib/http/agents.js";
import { formatDate } from "$lib/utils/format.js";

let { agent }: { agent: string } = $props();
const query = createQuery(() => agentQueryOptions(agent));
const enabled = $derived(agentsEnabled());
const runtime = $derived(query.data);
const ready = $derived(runtime?.status === "ready");
const connectionIssue = $derived.by(() => {
    if (!runtime) return null;
    try {
        runtimeBaseUrl(runtime);
        return null;
    } catch (error) {
        return getErrorMessage(error);
    }
});

let endpoint = $state("/");
let pending = $state(false);
let requestError = $state<string | null>(null);
let result = $state<{
    status: number;
    text: string;
    truncated: boolean;
} | null>(null);
let controller: AbortController | null = null;
let disposed = false;
onDestroy(() => {
    disposed = true;
    controller?.abort();
});

async function refresh() {
    if (disposed || !enabled || query.isFetching || pending) return;
    await query.refetch();
}

async function request(event: SubmitEvent) {
    event.preventDefault();
    const account = agentAccountKey();
    if (
        disposed ||
        !agentsEnabled() ||
        !account ||
        !runtime ||
        !ready ||
        pending ||
        connectionIssue ||
        query.isError
    )
        return;
    const requestedAgent = runtime;
    const isCurrent = () =>
        !disposed &&
        account === agentAccountKey() &&
        runtime?.name === requestedAgent.name &&
        runtime?.url === requestedAgent.url &&
        ready &&
        !query.isError;
    controller = new AbortController();
    pending = true;
    requestError = null;
    result = null;
    try {
        const response = await runtimeFetch(requestedAgent, endpoint, {
            method: "GET",
            signal: AbortSignal.any([
                controller.signal,
                AbortSignal.timeout(15_000),
            ]),
        });
        const reader = response.body?.getReader();
        const buffer = new Uint8Array(64 * 1024);
        let length = 0;
        let truncated = false;
        if (reader) {
            try {
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) break;
                    const remaining = buffer.length - length;
                    buffer.set(value.subarray(0, remaining), length);
                    length += Math.min(value.length, remaining);
                    if (length === buffer.length) {
                        truncated = true;
                        await reader.cancel();
                        break;
                    }
                }
            } finally {
                reader.releaseLock();
            }
        }
        if (isCurrent()) {
            result = {
                status: response.status,
                text: new TextDecoder().decode(buffer.subarray(0, length)),
                truncated,
            };
        }
    } catch (error) {
        if (isCurrent()) requestError = getErrorMessage(error);
    } finally {
        if (!disposed) pending = false;
        controller = null;
    }
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
      <Button variant="outline" disabled={!enabled || query.isFetching || pending} onclick={refresh}>
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
            description={runtime ? "The last fetched data is shown below and may be out of date. Refresh before sending requests." : "It may have been deleted, or the service may be unavailable."}
            onRetry={refresh}
            retrying={query.isFetching || pending}
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
          <Badge variant="outline">{agentStatusLabel(runtime.status)}</Badge>
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
          <div class="flex min-w-0 flex-col gap-1 sm:col-span-2">
            <dt class="text-muted-foreground">URL</dt>
            <dd class="break-all font-mono">{runtime.url || "Not available"}</dd>
          </div>
        </dl>
      </section>

      <section class="flex flex-col gap-4 border-t border-border pt-6">
        <h2>REST console</h2>
        <p class="max-w-2xl text-sm text-muted-foreground">
          Send an authenticated GET request to an endpoint supported by your agent.
          Paths are relative to the agent URL. A ready status does not guarantee that
          every endpoint is available.
        </p>
        {#if connectionIssue}
          <p role="status" class="text-sm text-muted-foreground">{connectionIssue}</p>
        {:else if !ready}
          <p role="status" class="text-sm text-muted-foreground">This agent is not ready for requests. Refresh to check its status.</p>
        {/if}
        <form class="flex flex-col gap-3" onsubmit={request} aria-busy={pending}>
          <label for="runtime-endpoint" class="text-sm font-medium">Endpoint path</label>
          <div class="flex flex-col gap-2 sm:flex-row">
            <Input id="runtime-endpoint" value={endpoint} oninput={(event) => (endpoint = event.currentTarget.value)} required autocomplete="off" spellcheck={false} disabled={pending} />
            <Button type="submit" disabled={!enabled || !ready || pending || Boolean(connectionIssue) || query.isError}>{pending ? "Requesting…" : "Send GET"}</Button>
          </div>
        </form>
        {#if requestError}
          <p role="alert" class="break-words text-sm text-destructive">{requestError}</p>
        {/if}
        {#if result}
          <div class="flex min-w-0 flex-col gap-2" role="status">
            <p class="text-sm font-medium">HTTP {result.status}</p>
            <pre class="max-h-96 overflow-auto rounded-lg bg-muted p-4 text-xs">{result.text || "(empty response)"}</pre>
            {#if result.truncated}
              <p class="text-xs text-muted-foreground">Response limited to the first 64 KiB.</p>
            {/if}
          </div>
        {/if}
      </section>
      {/if}
    {/if}
  </div>
</ScrollArea>
