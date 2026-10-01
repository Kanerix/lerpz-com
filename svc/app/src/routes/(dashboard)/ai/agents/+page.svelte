<script lang="ts">
import Icon from "@iconify/svelte";
import { Button, Input, ScrollArea } from "@lerpz/ui";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { onDestroy } from "svelte";
import { toast } from "svelte-sonner";
import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { createAgent, listAgentMemory } from "$lib/api/agents/agents.js";
import type {
    AgentMemoryChoice,
    CreateAgentRequest,
} from "$lib/api/models/index.js";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { showError } from "$lib/components/error-dialog/index.js";
import { ErrorState } from "$lib/components/error-state/index.js";
import { optionCardVariants } from "./agents-variants.js";

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
const agentPattern = "[a-z0-9]([a-z0-9\\-]{0,38}[a-z0-9])?";

let name = $state("");
let memoryMode = $state<AgentMemoryChoice>("new");
let cpuLimit = $state("");
let memoryLimit = $state("");
let creating = $state(false);
let createdAgent = $state<string | null>(null);
let unconfirmedAgent = $state<string | null>(null);
let disposed = false;

onDestroy(() => {
    disposed = true;
});

const validAgent = $derived(new RegExp(`^${agentPattern}$`).test(name));

const validResourceLimits = $derived(
    isValidResourceLimit(cpuLimit) && isValidResourceLimit(memoryLimit),
);
const matchingMemory = $derived(
    memoryQuery.data?.find((memory) => memory.agent_name === name),
);
const canUseExistingMemory = $derived(
    memoryQuery.isSuccess &&
        Boolean(
            matchingMemory &&
                !matchingMemory.in_use &&
                matchingMemory.status !== "unavailable",
        ),
);
const memoryReady = $derived(
    memoryMode === "none" ||
        (memoryQuery.isSuccess &&
            (memoryMode === "new" ? !matchingMemory : canUseExistingMemory)),
);
const formDisabled = $derived(creating || createdAgent !== null || !enabled);
const canCreate = $derived(
    !formDisabled && validAgent && validResourceLimits && memoryReady,
);

function isValidResourceLimit(value: string): boolean {
    if (value === "") return true;
    const limit = Number(value);
    return Number.isInteger(limit) && limit > 0 && limit <= 4294967295;
}

async function create(event: SubmitEvent) {
    event.preventDefault();
    if (disposed || !canCreate || creating) return;

    const submissionAccount = accountKey;
    if (!enabled || !submissionAccount) return;

    const queryKey = ["agents", submissionAccount];
    const request: CreateAgentRequest = {
        name,
        memory: memoryMode,
        ...(cpuLimit !== "" || memoryLimit !== ""
            ? {
                  resource_limits: {
                      ...(cpuLimit !== ""
                          ? { cpu_millicores: Number(cpuLimit) }
                          : {}),
                      ...(memoryLimit !== ""
                          ? { memory_mib: Number(memoryLimit) }
                          : {}),
                  },
              }
            : {}),
    };
    const isCurrentSubmission = () =>
        !disposed && submissionAccount === accountKey;

    creating = true;
    try {
        const response = await createAgent(request);
        if (response.status !== 201) throw response.data;
        if (!isCurrentSubmission()) return;
        createdAgent = request.name;
        unconfirmedAgent = null;
    } catch (err) {
        if (!isCurrentSubmission()) return;
        unconfirmedAgent = request.name;
        showError(err);
    } finally {
        // Keep invalidation on the submitting account, even if the page was replaced.
        const refetchType =
            submissionAccount === accountKey ? "active" : "none";
        await queryClient.invalidateQueries({ queryKey, refetchType });
        if (isCurrentSubmission()) creating = false;
    }

    if (!createdAgent || !isCurrentSubmission()) return;
    toast.success(`Agent "${createdAgent}" created`);
    try {
        await goto("/ai/agents/sessions");
    } catch (err) {
        if (isCurrentSubmission()) showError(err);
    }
}
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight">Create an agent</h1>
    <p class="text-sm text-muted-foreground">
      Give your agent a name and choose whether it keeps persistent memory.
    </p>
  </header>

  {#if !enabled}
    <p role="status" class="text-sm text-muted-foreground">Sign in to create an agent.</p>
  {:else if memoryMode !== "none" && memoryQuery.isError}
    <ErrorState
      compact
      title="Couldn't load agent memory"
      description="Refresh before creating or reusing memory, or choose No memory."
      retrying={memoryQuery.isFetching || creating}
      retryLabel="Refresh"
      onRetry={() => { void memoryQuery.refetch(); }}
    />
  {/if}

  {#if unconfirmedAgent}
    <div role="alert" class="flex flex-col gap-2 border-l-2 border-destructive pl-4 text-sm">
      <p class="font-medium">Agent creation may be incomplete.</p>
      <p class="text-muted-foreground">
        Creation of <code class="break-all">{unconfirmedAgent}</code> could not be confirmed.
        Any memory created has been retained. Check agent sessions and memory before trying again.
      </p>
      <div class="flex flex-wrap gap-2">
        <Button href="/ai/agents/sessions" variant="outline" size="sm">View sessions</Button>
        <Button href="/ai/agents/memory" variant="outline" size="sm">Manage memory</Button>
      </div>
    </div>
  {/if}

  {#if createdAgent}
    <div role="status" class="flex flex-col items-start gap-2 text-sm">
      <p>Agent <code class="break-all">{createdAgent}</code> has been created.</p>
      <Button href="/ai/agents/sessions" variant="outline" size="sm">View sessions</Button>
    </div>
  {/if}

  <form onsubmit={create} aria-busy={creating}>
    <fieldset disabled={formDisabled} class="flex min-w-0 flex-col gap-8">
      <section class="flex flex-col gap-4">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          Agent
        </h2>
        <div class="flex flex-col gap-2">
          <label for="agent-name" class="text-sm font-medium">Agent name</label>
          <Input
            id="agent-name"
            value={name}
            oninput={(e) => (name = e.currentTarget.value)}
            placeholder="research-assistant"
            autocomplete="off"
            autocapitalize="none"
            spellcheck={false}
            required
            maxlength={40}
            pattern={agentPattern}
            title="Use up to 40 lowercase letters, numbers or hyphens. Start and end with a letter or number."
            aria-describedby="agent-name-help"
          />
          <p id="agent-name-help" class="text-xs text-muted-foreground">
            Up to 40 lowercase letters, numbers or hyphens. Start and end with a letter or number.
            Saved memory must belong to this exact name.
          </p>
        </div>

      </section>

      <section class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <h2 id="memory-heading" class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
            Persistent memory
          </h2>
          <p class="text-xs text-muted-foreground">
            Keep saved memory across sessions. Deleting an agent keeps its memory.
          </p>
        </div>

        <div role="group" aria-labelledby="memory-heading" class="grid gap-2 sm:grid-cols-3">
          <Button
            type="button"
            variant="outline"
            aria-pressed={memoryMode === "new"}
            disabled={Boolean(matchingMemory)}
            onclick={() => (memoryMode = "new")}
            class={optionCardVariants({ selected: memoryMode === "new" })}
          >
            <Icon icon="fa6-solid:plus" class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
            <span class="flex flex-col gap-0.5">
              <span class="text-sm font-medium">New memory</span>
              <span class="text-xs text-muted-foreground">Create persistent memory.</span>
            </span>
          </Button>
          <Button
            type="button"
            variant="outline"
            aria-pressed={memoryMode === "existing"}
            disabled={!canUseExistingMemory}
            onclick={() => (memoryMode = "existing")}
            class={optionCardVariants({ selected: memoryMode === "existing" })}
          >
            <Icon icon="fa6-solid:database" class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
            <span class="flex flex-col gap-0.5">
              <span class="text-sm font-medium">Use existing</span>
              <span class="text-xs text-muted-foreground">Reuse this agent's memory.</span>
            </span>
          </Button>
          <Button
            type="button"
            variant="outline"
            aria-pressed={memoryMode === "none"}
            onclick={() => (memoryMode = "none")}
            class={optionCardVariants({ selected: memoryMode === "none" })}
          >
            <Icon icon="fa6-solid:ban" class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
            <span class="flex flex-col gap-0.5">
              <span class="text-sm font-medium">No memory</span>
              <span class="text-xs text-muted-foreground">Run without saved memory.</span>
            </span>
          </Button>
        </div>

        {#if memoryMode === "none"}
          <p class="text-xs text-muted-foreground">
            No persistent memory will be used. Any existing memory is left untouched.
          </p>
        {/if}

        {#if enabled && memoryMode !== "none"}
          <div aria-live="polite" class="flex flex-col gap-2 text-sm">
            {#if memoryQuery.isPending}
              <p class="flex items-center gap-2 text-muted-foreground">
                <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
                Loading agent memory…
              </p>
            {:else if !validAgent}
              <p class="text-muted-foreground">Enter a valid agent name to check its memory.</p>
            {:else if matchingMemory}
              <p class="break-all font-mono">{matchingMemory.agent_name}</p>
              <p class="text-xs text-muted-foreground">
                Status: <span class="capitalize">{matchingMemory.status.replaceAll("_", " ")}</span>
                {#if matchingMemory.in_use} · In use{/if}
              </p>
              {#if matchingMemory.in_use}
                <p class="text-destructive">This memory is in use and cannot be reused by a new agent.</p>
              {:else if matchingMemory.status === "unavailable"}
                <p class="text-destructive">This memory is unavailable and cannot be reused.</p>
              {:else if matchingMemory.status === "pending"}
                <p class="text-xs text-muted-foreground">Pending memory can be used to create an agent.</p>
              {/if}
              {#if memoryMode === "new"}
                <p class="text-destructive">
                  Memory already exists for this name. Reuse it if available, choose No memory, or use another name.
                </p>
              {/if}
            {:else if memoryQuery.isSuccess && memoryMode === "existing"}
              <p class="text-destructive">
                No saved memory was found for <code class="break-all">{name}</code>.
                Choose New memory or No memory. Another agent's memory cannot be used.
              </p>
            {/if}
          </div>
        {/if}
      </section>

      <details>
        <summary class="cursor-pointer rounded-sm text-sm font-medium focus-visible:ring-[3px] focus-visible:ring-ring/50">
          Advanced resource limits
        </summary>
        <div class="flex flex-col gap-4 pt-4">
          <p id="resource-limits-help" class="text-xs text-muted-foreground">
            Optional. Leave blank to use the defaults, or enter whole numbers from 1 to 4294967295.
            1000 millicores is one CPU core. RAM is separate from persistent agent memory.
          </p>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="flex flex-col gap-2">
              <label for="cpu-limit" class="text-sm font-medium">CPU limit (millicores)</label>
              <Input
                id="cpu-limit"
                type="number"
                min="1"
                max="4294967295"
                step="1"
                value={cpuLimit}
                oninput={(e) => (cpuLimit = e.currentTarget.value)}
                placeholder="Default"
                aria-describedby="resource-limits-help"
                aria-invalid={!isValidResourceLimit(cpuLimit)}
              />
            </div>
            <div class="flex flex-col gap-2">
              <label for="memory-limit" class="text-sm font-medium">RAM limit (MiB)</label>
              <Input
                id="memory-limit"
                type="number"
                min="1"
                max="4294967295"
                step="1"
                value={memoryLimit}
                oninput={(e) => (memoryLimit = e.currentTarget.value)}
                placeholder="Default"
                aria-describedby="resource-limits-help"
                aria-invalid={!isValidResourceLimit(memoryLimit)}
              />
            </div>
          </div>
          {#if !validResourceLimits}
            <p role="alert" class="text-xs text-destructive">Resource limits must be whole numbers from 1 to 4294967295, or left blank.</p>
          {/if}
        </div>
      </details>

      <div class="flex items-center justify-end gap-3 border-t pt-6">
        {#if creating}
          <Button type="button" variant="ghost" disabled>Cancel</Button>
        {:else}
          <Button href="/ai/agents/sessions" variant="ghost">Cancel</Button>
        {/if}
        <Button type="submit" disabled={!canCreate}>
          {#if creating}
            <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
            Creating…
          {:else}
            <Icon icon="fa6-solid:rocket" class="size-4" />
            Create agent
          {/if}
        </Button>
      </div>
    </fieldset>
  </form>
</div>
</ScrollArea>
