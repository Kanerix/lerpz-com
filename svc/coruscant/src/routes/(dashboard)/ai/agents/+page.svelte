<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { Input } from "@lerpz/ui/components/input";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
} from "@lerpz/ui/components/select";
import { toast } from "svelte-sonner";

// DRAFT: this page scaffolds the "deploy a new agent" flow. Submitting should
// call `createAgent` from `$lib/api/agents/agents.ts` once the kamino OpenAPI
// spec exposes the request body (name, model, memory volume, resources). For
// now it validates locally and shows a toast so the layout can be reviewed.

type MemoryMode = "new" | "existing";

let name = $state("");
let model = $state("gpt-4o");
let namespace = $state("agents");
let memoryMode = $state<MemoryMode>("new");
let volumeSize = $state("5");
let existingVolume = $state("");
let cpu = $state("500m");
let memory = $state("1Gi");
let deploying = $state(false);

// Placeholder catalogue. Replace with the models hook / PVC list once wired up.
const models = [
    { value: "gpt-4o", label: "GPT-4o" },
    { value: "claude-3-7-sonnet", label: "Claude 3.7 Sonnet" },
    { value: "llama-3.1-70b", label: "Llama 3.1 70B" },
];

const existingVolumes = [
    { value: "mem-research-01", label: "mem-research-01 · 10Gi" },
    { value: "mem-support-01", label: "mem-support-01 · 5Gi" },
];

const modelLabel = $derived(
    models.find((m) => m.value === model)?.label ?? "Select a model",
);
const existingVolumeLabel = $derived(
    existingVolumes.find((v) => v.value === existingVolume)?.label ??
        "Select a volume",
);

const canDeploy = $derived(
    name.trim().length > 0 &&
        (memoryMode === "new"
            ? Number(volumeSize) > 0
            : existingVolume.length > 0),
);

async function deploy(event: SubmitEvent) {
    event.preventDefault();
    if (!canDeploy || deploying) return;
    deploying = true;
    try {
        // TODO: await createAgent({ ... }) — provisions a Deployment + PVC.
        await new Promise((resolve) => setTimeout(resolve, 700));
        toast.success(`Agent "${name}" queued for deployment`, {
            description: `Namespace ${namespace} · ${
                memoryMode === "new"
                    ? `new ${volumeSize}Gi memory volume`
                    : existingVolume
            }`,
        });
        name = "";
    } catch (err) {
        toast.error("Couldn't deploy agent", {
            description:
                err instanceof Error ? err.message : "Please try again.",
        });
    } finally {
        deploying = false;
    }
}
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-2xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight">Deploy a new agent</h1>
    <p class="text-sm text-muted-foreground">
      An agent runs as a container in the cluster and is backed by a persistent
      memory volume it can read and write to across sessions.
    </p>
  </header>

  <form class="flex flex-col gap-8" onsubmit={deploy}>
    <!-- Identity -->
    <section class="flex flex-col gap-4">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        Identity
      </h2>
      <div class="flex flex-col gap-2">
        <label for="agent-name" class="text-sm font-medium">Name</label>
        <Input
          id="agent-name"
          value={name}
          oninput={(e) => (name = e.currentTarget.value)}
          placeholder="research-assistant"
          autocomplete="off"
        />
        <p class="text-xs text-muted-foreground">
          Used for the Deployment and service names. Lowercase, no spaces.
        </p>
      </div>

      <div class="grid gap-4 sm:grid-cols-2">
        <div class="flex flex-col gap-2">
          <span class="text-sm font-medium">Base model</span>
          <Select
            items={models}
            value={model}
            onValueChange={(v) => {
              if (v) model = v;
            }}
          >
            <SelectTrigger>{modelLabel}</SelectTrigger>
            <SelectContent>
              {#each models as m (m.value)}
                <SelectItem value={m.value}>{m.label}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
        <div class="flex flex-col gap-2">
          <label for="agent-namespace" class="text-sm font-medium">Namespace</label>
          <Input
            id="agent-namespace"
            value={namespace}
            oninput={(e) => (namespace = e.currentTarget.value)}
            placeholder="agents"
            autocomplete="off"
          />
        </div>
      </div>
    </section>

    <!-- Memory -->
    <section class="flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
          Memory volume
        </h2>
        <p class="text-xs text-muted-foreground">
          Where the agent stores long-term memory. Provisioned as a
          PersistentVolumeClaim.
        </p>
      </div>

      <div class="grid gap-2 sm:grid-cols-2">
        <button
          type="button"
          onclick={() => (memoryMode = "new")}
          class="flex items-start gap-3 rounded-xl border p-4 text-left transition-colors
            {memoryMode === 'new'
              ? 'border-primary bg-primary/5'
              : 'border-border hover:bg-muted/50'}"
        >
          <Icon icon="fa6-solid:plus" class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
          <span class="flex flex-col gap-0.5">
            <span class="text-sm font-medium">New volume</span>
            <span class="text-xs text-muted-foreground">Provision a fresh PVC.</span>
          </span>
        </button>
        <button
          type="button"
          onclick={() => (memoryMode = "existing")}
          class="flex items-start gap-3 rounded-xl border p-4 text-left transition-colors
            {memoryMode === 'existing'
              ? 'border-primary bg-primary/5'
              : 'border-border hover:bg-muted/50'}"
        >
          <Icon icon="fa6-solid:database" class="mt-0.5 size-4 shrink-0 text-muted-foreground" />
          <span class="flex flex-col gap-0.5">
            <span class="text-sm font-medium">Attach existing</span>
            <span class="text-xs text-muted-foreground">Reuse a memory volume.</span>
          </span>
        </button>
      </div>

      {#if memoryMode === "new"}
        <div class="flex flex-col gap-2">
          <label for="volume-size" class="text-sm font-medium">Size (Gi)</label>
          <Input
            id="volume-size"
            type="number"
            min="1"
            value={volumeSize}
            oninput={(e) => (volumeSize = e.currentTarget.value)}
          />
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          <span class="text-sm font-medium">Existing volume</span>
          <Select
            items={existingVolumes}
            value={existingVolume}
            onValueChange={(v) => {
              if (v) existingVolume = v;
            }}
          >
            <SelectTrigger>{existingVolumeLabel}</SelectTrigger>
            <SelectContent>
              {#each existingVolumes as v (v.value)}
                <SelectItem value={v.value}>{v.label}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>
      {/if}
    </section>

    <!-- Resources -->
    <section class="flex flex-col gap-4">
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        Resources
      </h2>
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="flex flex-col gap-2">
          <label for="cpu" class="text-sm font-medium">CPU request</label>
          <Input
            id="cpu"
            value={cpu}
            oninput={(e) => (cpu = e.currentTarget.value)}
            placeholder="500m"
          />
        </div>
        <div class="flex flex-col gap-2">
          <label for="memory" class="text-sm font-medium">Memory request</label>
          <Input
            id="memory"
            value={memory}
            oninput={(e) => (memory = e.currentTarget.value)}
            placeholder="1Gi"
          />
        </div>
      </div>
    </section>

    <div class="flex items-center justify-end gap-3 border-t pt-6">
      <Button href="/ai/agents/sessions" variant="ghost">Cancel</Button>
      <Button type="submit" disabled={!canDeploy || deploying}>
        {#if deploying}
          <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
          Deploying…
        {:else}
          <Icon icon="fa6-solid:rocket" class="size-4" />
          Deploy agent
        {/if}
      </Button>
    </div>
  </form>
</div>
</ScrollArea>
