<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { site } from "$lib/config.js";

const services = [
    {
        name: "app",
        role: "What you actually log into.",
        stack: "SvelteKit",
    },
    {
        name: "api",
        role: "Does the work the app asks for.",
        stack: "Rust · Axum",
    },
    {
        name: "artoo",
        role: "Runs the agents and lets them act.",
        stack: "Rust",
    },
    {
        name: "forge",
        role: "Gives each agent its memory and compute.",
        stack: "Rust · Kubernetes",
    },
    {
        name: "www",
        role: "This site. Just files, no server.",
        stack: "SvelteKit",
    },
];
</script>

<section class="flex flex-col gap-8 py-16 md:py-24">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">How it's built</h2>
    <p class="leading-relaxed text-muted-foreground">
      Five services, each doing one job. It's easier to fix that way, and easier
      to explain.
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
    Read the code
    <Icon icon="fa6-brands:github" class="size-4" />
  </Button>
</section>
