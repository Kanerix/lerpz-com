<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { resolve } from "$app/paths";
import { site } from "$lib/config.js";

const principles = [
    {
        title: "Own the whole stack",
        description:
            "Interface, API, agents, and the infrastructure underneath are all mine. Nothing important sits in a black box I cannot open.",
    },
    {
        title: "Small services, clear jobs",
        description:
            "Every service does one thing. It keeps the system understandable, and it means a change in one place rarely breaks another.",
    },
    {
        title: "Ship it, then sharpen it",
        description:
            "Working software beats a perfect plan. I get things in front of people early and keep refining once I know how they are used.",
    },
];
</script>

<svelte:head>
  <title>About — {site.name}</title>
  <meta
    name="description"
    content="How Lerpz AI came about, how the platform is built, and the principles behind it."
  />
</svelte:head>

<section class="flex flex-col gap-4 py-8 md:py-12">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">About</p>
  <h1 class="max-w-3xl text-4xl font-semibold tracking-tight md:text-5xl">
    Built because the alternative was a dozen subscriptions.
  </h1>
  <p class="max-w-2xl text-lg leading-relaxed text-muted-foreground">
    Most organisations end up with AI scattered everywhere — a chat tool here, an
    image generator there, each with its own login, its own bill, and its own
    idea of who is allowed to use it. Lerpz AI exists to replace that with one
    platform that is actually maintained.
  </p>
</section>

<section class="flex flex-col gap-8 py-8 md:py-12">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">How I work</h2>
    <p class="leading-relaxed text-muted-foreground">
      Three things that shape most of the decisions here.
    </p>
  </div>

  <ol class="flex flex-col">
    {#each principles as principle, i (principle.title)}
      <li class="grid gap-2 border-t border-border/60 py-6 sm:grid-cols-[4rem_1fr] sm:gap-8">
        <span class="font-mono text-sm text-primary">
          {String(i + 1).padStart(2, "0")}
        </span>
        <div class="flex flex-col gap-2">
          <h3 class="text-base font-semibold">{principle.title}</h3>
          <p class="max-w-2xl leading-relaxed text-muted-foreground">
            {principle.description}
          </p>
        </div>
      </li>
    {/each}
  </ol>
</section>

<section class="flex flex-col gap-5 rounded-xl border bg-card/40 p-6 md:p-8">
  <h2 class="text-2xl font-semibold tracking-tight">Want to see more?</h2>
  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    The portfolio has the wider body of work behind Lerpz, and the source for
    this platform is public. If you would rather just ask a question, that works
    too.
  </p>
  <div class="flex flex-wrap items-center gap-3">
    <Button href={site.portfolioUrl} target="_blank" rel="noopener noreferrer">
      View the portfolio
      <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-3.5" />
    </Button>
    <Button variant="outline" href={resolve("/contact")}>
      Contact me
      <Icon icon="fa6-solid:arrow-right" class="size-3.5" />
    </Button>
  </div>
</section>
