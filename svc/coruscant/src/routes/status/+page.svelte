<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import { Button } from "@lerpz/ui/components/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import { Skeleton } from "@lerpz/ui/components/skeleton";
import { createQuery } from "@tanstack/svelte-query";
import { getHealthCheckUrl } from "$lib/api/health/health.js";
import type { HealthCheck } from "$lib/api/models/index.js";
import Footer from "$lib/components/landing-page/Footer.svelte";
import Header from "$lib/components/landing-page/Header.svelte";
import { authenticatedFetch } from "$lib/http/fetch.js";

type ServiceStatus = "operational" | "outage" | "unknown";

// The health endpoint returns 200 when everything is healthy and 503 when a
// dependency is down — but *both* responses carry the same HealthCheck body
// describing each component. Only unexpected statuses (e.g. 500) or network
// failures are treated as real errors. We bypass the generated client here
// because its shared fetch mutator throws away the body on any non-2xx status.
async function fetchHealth(signal: AbortSignal): Promise<HealthCheck> {
    const response = await authenticatedFetch(getHealthCheckUrl(), { signal });
    if (response.status === 200 || response.status === 503) {
        return (await response.json()) as HealthCheck;
    }
    throw new Error(`Health check failed with status ${response.status}`);
}

const query = createQuery(() => ({
    queryKey: [getHealthCheckUrl()],
    queryFn: ({ signal }: { signal: AbortSignal }) => fetchHealth(signal),
    refetchInterval: 30_000,
}));

const statusMeta: Record<
    ServiceStatus,
    {
        label: string;
        icon: string;
        badgeVariant: "default" | "secondary" | "destructive" | "outline";
        dot: string;
        text: string;
    }
> = {
    operational: {
        label: "Operational",
        icon: "fa6-solid:circle-check",
        badgeVariant: "default",
        dot: "bg-emerald-500",
        text: "text-emerald-500",
    },
    outage: {
        label: "Outage",
        icon: "fa6-solid:circle-xmark",
        badgeVariant: "destructive",
        dot: "bg-destructive",
        text: "text-destructive",
    },
    unknown: {
        label: "Unknown",
        icon: "fa6-solid:circle-question",
        badgeVariant: "secondary",
        dot: "bg-muted-foreground",
        text: "text-muted-foreground",
    },
};

const components: { key: keyof HealthCheck; name: string; description: string; icon: string }[] = [
    {
        key: "database",
        name: "Database",
        description: "Primary PostgreSQL data store.",
        icon: "fa6-solid:database",
    },
    {
        key: "redis",
        name: "Cache",
        description: "Redis cache and session store.",
        icon: "fa6-solid:bolt",
    },
    {
        key: "s3",
        name: "Object Storage",
        description: "S3-compatible file and image storage.",
        icon: "fa6-solid:cloud",
    },
];

const health = $derived(query.data);

// A thrown query error means the health endpoint itself was unreachable or
// returned an unexpected status, so we genuinely don't know the components.
const apiUp = $derived(!query.isError);

function componentStatus(key: keyof HealthCheck): ServiceStatus {
    if (query.isLoading || !apiUp || !health) return "unknown";
    return health[key] ? "operational" : "outage";
}

const services = $derived(
    components.map((c) => ({ ...c, status: componentStatus(c.key) })),
);

const worstStatus = $derived.by<ServiceStatus>(() => {
    if (query.isLoading) return "unknown";
    if (!apiUp) return "outage";
    return services.some((s) => s.status === "outage") ? "outage" : "operational";
});

const summary = $derived.by(() => {
    if (query.isLoading) return "Checking system status…";
    if (!apiUp) return "API unreachable";
    if (worstStatus === "outage") return "Some systems are down";
    return "All systems operational";
});

const lastUpdated = $derived.by(() => {
    const at = Math.max(query.dataUpdatedAt, query.errorUpdatedAt);
    return at
        ? new Date(at).toLocaleString("en-GB", {
              dateStyle: "medium",
              timeStyle: "short",
          })
        : null;
});
</script>

<svelte:head>
  <title>Status — Lerpz AI</title>
  <meta name="description" content="Live status of Lerpz AI services." />
</svelte:head>

<div class="relative flex min-h-screen flex-col">
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 -z-10 flex items-start justify-center"
  >
    <div class="mt-16 h-150 w-225 rounded-full dark:bg-primary/10 blur-3xl"></div>
  </div>
  <Header />

  <main class="mx-auto w-full max-w-5xl flex-1 px-4 py-12 md:py-16">
    <section class="flex flex-col gap-6 py-8 md:py-12">
      <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">System Status</p>

      <div class="flex flex-col gap-4 rounded-xl border p-6 sm:flex-row sm:items-center sm:justify-between">
        <div class="flex items-center gap-4">
          <span class="relative flex size-3">
            {#if query.isLoading}
              <span class="relative inline-flex size-3 rounded-full {statusMeta[worstStatus].dot}"></span>
            {:else}
              <span
                class="absolute inline-flex h-full w-full animate-ping rounded-full opacity-75 {statusMeta[worstStatus].dot}"
              ></span>
              <span class="relative inline-flex size-3 rounded-full {statusMeta[worstStatus].dot}"></span>
            {/if}
          </span>
          <div class="flex flex-col gap-0.5">
            <h1 class="text-2xl font-semibold tracking-tight {statusMeta[worstStatus].text}">
              {summary}
            </h1>
            <p class="text-sm text-muted-foreground">
              {#if lastUpdated}
                Last checked {lastUpdated}
              {:else}
                Fetching latest data…
              {/if}
            </p>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <Badge variant={statusMeta[worstStatus].badgeVariant} class="w-fit gap-1.5">
            <Icon icon={statusMeta[worstStatus].icon} class="size-3.5" />
            {statusMeta[worstStatus].label}
          </Badge>
          <Button
            variant="outline"
            size="sm"
            onclick={() => query.refetch()}
            disabled={query.isFetching}
          >
            <Icon
              icon="fa6-solid:arrows-rotate"
              class="size-3.5 {query.isFetching ? 'animate-spin' : ''}"
            />
            Refresh
          </Button>
        </div>
      </div>
    </section>

    <section class="flex flex-col gap-6 py-8 md:py-12">
      <div class="flex flex-col gap-2">
        <h2 class="text-2xl font-semibold tracking-tight">Services</h2>
        <p class="text-muted-foreground">Live status of the core components powering Lerpz AI.</p>
      </div>

      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        {#each services as service (service.key)}
          {@const meta = statusMeta[service.status]}
          <Card>
            <CardHeader>
              <div class="mb-3 flex items-center justify-between gap-3">
                <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
                  <Icon icon={service.icon} class="size-5" />
                </div>
                {#if query.isLoading}
                  <Skeleton class="h-5 w-24 rounded-full" />
                {:else}
                  <Badge variant={meta.badgeVariant} class="gap-1.5">
                    <Icon icon={meta.icon} class="size-3" />
                    {meta.label}
                  </Badge>
                {/if}
              </div>
              <CardTitle class="text-base font-semibold">{service.name}</CardTitle>
              <CardDescription class="text-sm leading-relaxed">{service.description}</CardDescription>
            </CardHeader>
          </Card>
        {/each}
      </div>

      {#if !query.isLoading && !apiUp}
        <p class="rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          We couldn't reach the health endpoint. Component statuses may be inaccurate.
        </p>
      {/if}
    </section>
  </main>

  <Footer />
</div>
