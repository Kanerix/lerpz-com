<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import { Button } from "@lerpz/ui/components/button";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";

// DRAFT: static volume data. Wire up to the cluster's PersistentVolumeClaim
// listing so this reflects real memory volumes. Each volume holds an agent's
// long-term memory and can be attached to one agent at a time.

type VolumeStatus = "bound" | "available" | "pending";

type MemoryVolume = {
    name: string;
    status: VolumeStatus;
    capacityGi: number;
    usedGi: number;
    storageClass: string;
    boundTo: string | null;
    updated: string;
};

const volumes: MemoryVolume[] = [
    {
        name: "mem-research-01",
        status: "bound",
        capacityGi: 10,
        usedGi: 6.4,
        storageClass: "fast-ssd",
        boundTo: "research-assistant",
        updated: "2m ago",
    },
    {
        name: "mem-support-01",
        status: "bound",
        capacityGi: 5,
        usedGi: 1.2,
        storageClass: "standard",
        boundTo: "support-triage",
        updated: "1h ago",
    },
    {
        name: "mem-code-01",
        status: "pending",
        capacityGi: 20,
        usedGi: 0,
        storageClass: "fast-ssd",
        boundTo: "code-reviewer",
        updated: "just now",
    },
    {
        name: "mem-archive-01",
        status: "available",
        capacityGi: 50,
        usedGi: 38.1,
        storageClass: "standard",
        boundTo: null,
        updated: "3d ago",
    },
];

const statusMeta: Record<
    VolumeStatus,
    { label: string; dot: string; variant: "secondary" | "outline" }
> = {
    bound: { label: "Bound", dot: "bg-green-500", variant: "secondary" },
    available: { label: "Available", dot: "bg-sky-500", variant: "outline" },
    pending: { label: "Pending", dot: "bg-amber-500", variant: "outline" },
};

const totalCapacity = $derived(
    volumes.reduce((sum, v) => sum + v.capacityGi, 0),
);
const totalUsed = $derived(volumes.reduce((sum, v) => sum + v.usedGi, 0));

function usagePct(v: MemoryVolume): number {
    if (v.capacityGi === 0) return 0;
    return Math.min(100, Math.round((v.usedGi / v.capacityGi) * 100));
}
</script>

<ScrollArea class="h-full" orientation="vertical">
<div class="mx-auto flex w-full max-w-5xl flex-col gap-8 px-4 py-8">
  <header class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
    <div class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Memory</h1>
      <p class="text-sm text-muted-foreground">
        Persistent volumes that store what your agents remember across sessions.
      </p>
    </div>
    <Button>
      <Icon icon="fa6-solid:plus" class="size-3.5" />
      New volume
    </Button>
  </header>

  <!-- Summary -->
  <div class="grid gap-3 sm:grid-cols-3">
    <div class="rounded-xl border border-border bg-card p-4">
      <p class="text-xs uppercase tracking-wide text-muted-foreground">Volumes</p>
      <p class="mt-1 text-2xl font-semibold">{volumes.length}</p>
    </div>
    <div class="rounded-xl border border-border bg-card p-4">
      <p class="text-xs uppercase tracking-wide text-muted-foreground">Provisioned</p>
      <p class="mt-1 text-2xl font-semibold">{totalCapacity} Gi</p>
    </div>
    <div class="rounded-xl border border-border bg-card p-4">
      <p class="text-xs uppercase tracking-wide text-muted-foreground">Used</p>
      <p class="mt-1 text-2xl font-semibold">{totalUsed.toFixed(1)} Gi</p>
    </div>
  </div>

  <!-- Volumes -->
  <div class="flex flex-col gap-3">
    {#each volumes as volume (volume.name)}
      {@const meta = statusMeta[volume.status]}
      {@const pct = usagePct(volume)}
      <div class="flex flex-col gap-4 rounded-xl border border-border bg-card p-4 sm:flex-row sm:items-center sm:justify-between">
        <div class="flex min-w-0 flex-col gap-2 sm:w-2/3">
          <div class="flex items-center gap-2">
            <span class="size-2 shrink-0 rounded-full {meta.dot}"></span>
            <span class="truncate font-mono text-sm font-medium">{volume.name}</span>
            <Badge variant={meta.variant}>{meta.label}</Badge>
          </div>
          <div class="flex flex-col gap-1">
            <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
              <div
                class="h-full rounded-full bg-primary transition-all"
                style="width: {pct}%"
              ></div>
            </div>
            <div class="flex items-center justify-between text-xs text-muted-foreground">
              <span>{volume.usedGi} Gi of {volume.capacityGi} Gi</span>
              <span>{pct}%</span>
            </div>
          </div>
        </div>

        <div class="flex items-center justify-between gap-4 sm:justify-end">
          <div class="flex flex-col gap-1 text-xs text-muted-foreground">
            <span class="inline-flex items-center gap-1.5">
              <Icon icon="fa6-solid:layer-group" class="size-3" />
              {volume.storageClass}
            </span>
            <span class="inline-flex items-center gap-1.5">
              <Icon icon="fa6-solid:robot" class="size-3" />
              {volume.boundTo ?? "Unattached"}
            </span>
          </div>
          <Button variant="outline" size="sm">Manage</Button>
        </div>
      </div>
    {/each}
  </div>
</div>
</ScrollArea>
