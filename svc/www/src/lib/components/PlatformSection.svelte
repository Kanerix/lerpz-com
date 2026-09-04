<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { site } from "$lib/config.js";

const services = [
    {
        name: "app",
        role: "The product interface everyone signs into.",
        stack: "SvelteKit",
    },
    {
        name: "api",
        role: "Chat, generation, models, and everything the app asks for.",
        stack: "Rust · Axum",
    },
    {
        name: "artoo",
        role: "The agent runtime — reasons over a task and acts on it.",
        stack: "Rust",
    },
    {
        name: "forge",
        role: "Provisions the memory and compute an agent needs to run.",
        stack: "Rust · Kubernetes",
    },
    {
        name: "www",
        role: "This site. Static files, no server, no tracking.",
        stack: "SvelteKit",
    },
];
</script>

<section class="flex flex-col gap-8 py-16 md:py-24">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">How it's built</h2>
    <p class="leading-relaxed text-muted-foreground">
      Lerpz runs as a handful of small, single-purpose services. Each one does a
      single job well, which keeps the platform easy to reason about and easy to
      change.
    </p>
  </div>

  <ul class="flex flex-col">
    {#each services as service (service.name)}
      <li
        class="grid gap-1 border-t border-border/60 py-5 sm:grid-cols-[7rem_1fr_auto] sm:items-baseline sm:gap-6"
      >
        <span class="font-mono text-sm text-primary">{service.name}</span>
        <span class="text-sm leading-relaxed text-muted-foreground">{service.role}</span>
        <span class="text-xs uppercase tracking-wider text-muted-foreground/70">
          {service.stack}
        </span>
      </li>
    {/each}
  </ul>

  <Button
    variant="outline"
    class="w-fit"
    href={site.githubUrl}
    target="_blank"
    rel="noopener noreferrer"
  >
    Browse the source
    <Icon icon="fa6-brands:github" class="size-4" />
  </Button>
</section>
