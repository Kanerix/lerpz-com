<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { resolve } from "$app/paths";
import { site } from "$lib/config.js";

const stack = [
    { name: "app", role: "Product interface" },
    { name: "api", role: "Core services" },
    { name: "artoo", role: "Autonomous agents" },
    { name: "forge", role: "Agent infrastructure" },
];
</script>

<section
  class="grid gap-12 py-16 md:py-24 lg:grid-cols-[minmax(0,1fr)_20rem] lg:items-center lg:gap-16"
>
  <div class="flex flex-col gap-6">
    <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Lerpz</p>

    <h1 class="max-w-3xl text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl">
      I build the AI platform,<br class="hidden sm:block" />
      <span class="text-primary">so your teams don't have to.</span>
    </h1>

    <p class="max-w-xl text-lg leading-relaxed text-muted-foreground">
      Lerpz AI is a small set of services that give an organisation one place for
      chat, generation, and agents — designed, built, and operated end to end by
      one person, rather than assembled from a dozen subscriptions.
    </p>

    <div class="flex flex-wrap items-center gap-3 pt-2">
      <Button size="lg" href={site.appUrl}>
        Open the app
        <Icon icon="fa6-solid:arrow-right" class="size-4" />
      </Button>
      <Button variant="ghost" size="lg" href={resolve("/#platform")}>
        See what it does
      </Button>
    </div>
  </div>

  <div class="rounded-xl border bg-card/40 p-5">
    <div class="flex items-center justify-between border-b border-border/60 pb-3">
      <span class="text-xs uppercase tracking-widest text-muted-foreground">The stack</span>
      <span class="size-2 rounded-full bg-primary"></span>
    </div>
    <ul class="flex flex-col gap-3.5 pt-4">
      {#each stack as service (service.name)}
        <li class="flex items-baseline justify-between gap-4">
          <span class="font-mono text-sm text-foreground">{service.name}</span>
          <span class="text-xs text-muted-foreground">{service.role}</span>
        </li>
      {/each}
    </ul>
  </div>
</section>
