<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { resolve } from "$app/paths";
import { site } from "$lib/config.js";

const principles = [
    {
        title: "Own the whole stack",
        description:
            "I wrote the interface, the API, the agents and the infrastructure. If something breaks, I can open it up and look.",
    },
    {
        title: "Small services, clear jobs",
        description:
            "Each service does one job. It means I can change one thing without holding my breath.",
    },
    {
        title: "Ship it, then sharpen it",
        description:
            "I'd rather put something usable in front of people early than plan it to death. It gets better once I see how it's actually used.",
    },
];
</script>

<svelte:head>
  <title>About — {site.name}</title>
  <meta
    name="description"
    content="Why Lerpz AI exists, how it's put together, and who builds it."
  />
</svelte:head>

<section class="flex flex-col gap-4 py-8 md:py-12">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">About</p>
  <h1 class="max-w-3xl text-4xl font-semibold tracking-tight md:text-5xl">
    I got tired of paying for five different AI tools.
  </h1>
  <p class="max-w-2xl text-lg leading-relaxed text-muted-foreground">
    Most places end up with AI scattered all over. A chat tool here, an image
    generator there, separate logins and separate bills, and nobody quite sure
    who's allowed to use what. Lerpz AI is my attempt at replacing that with one
    thing that actually gets maintained.
  </p>
</section>

<section class="flex flex-col gap-8 py-8 md:py-12">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">How I work</h2>
    <p class="leading-relaxed text-muted-foreground">
      Three things I keep coming back to.
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
  <h2 class="text-2xl font-semibold tracking-tight">Want to know who's behind it?</h2>
  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    My CV has the official version: jobs, education, where I've worked. The code
    is public too, if you'd rather just read that. Or you can ask me something.
  </p>
  <div class="flex flex-wrap items-center gap-3">
    <Button href={site.portfolioUrl} target="_blank" rel="noopener noreferrer">
      View my CV
      <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-3.5" />
    </Button>
    <Button variant="outline" href={resolve("/contact")}>
      Contact me
      <Icon icon="fa6-solid:arrow-right" class="size-3.5" />
    </Button>
  </div>
</section>
