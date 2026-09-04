<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { site } from "$lib/config.js";

const mailto = `mailto:${site.contactEmail}`;

const channels = [
    {
        icon: "fa6-solid:envelope",
        title: "Email me",
        description: "The fastest way to reach me for questions or support.",
        actionLabel: site.contactEmail,
        href: mailto,
    },
    {
        icon: "fa6-brands:github",
        title: "GitHub",
        description:
            "Browse my projects, report issues, or follow along with development.",
        actionLabel: "github.com/lerpz",
        href: site.githubUrl,
        external: true,
    },
    {
        icon: "fa6-solid:briefcase",
        title: "Portfolio",
        description:
            "See the wider body of work behind Lerpz, beyond the AI platform itself.",
        actionLabel: "portfolio.lerpz.com",
        href: site.portfolioUrl,
        external: true,
    },
];
</script>

<svelte:head>
  <title>Contact — {site.name}</title>
  <meta name="description" content="Get in touch with the Lerpz AI team." />
</svelte:head>

<section class="flex flex-col gap-4 py-8 md:py-12">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Contact</p>
  <h1 class="max-w-2xl text-4xl font-semibold tracking-tight md:text-5xl">
    Let's talk.
  </h1>
  <p class="max-w-2xl text-lg leading-relaxed text-muted-foreground">
    Have a question, a request, or feedback about Lerpz AI? Reach out and I'll get back to
    you as soon as I can.
  </p>
</section>

<section class="grid grid-cols-1 gap-4 py-8 md:grid-cols-3 md:py-12">
  {#each channels as channel (channel.title)}
    <div class="flex flex-col gap-3 rounded-xl border p-6">
      <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
        <Icon icon={channel.icon} class="size-5" />
      </div>
      <h2 class="text-base font-semibold">{channel.title}</h2>
      <p class="text-sm leading-relaxed text-muted-foreground">{channel.description}</p>
      <a
        href={channel.href}
        target={channel.external ? "_blank" : undefined}
        rel={channel.external ? "noopener noreferrer" : undefined}
        class="mt-1 inline-flex w-fit items-center gap-2 text-sm font-medium text-primary transition-opacity hover:opacity-80"
      >
        {channel.actionLabel}
        <Icon icon="fa6-solid:arrow-right" class="size-3" />
      </a>
    </div>
  {/each}
</section>

<section class="flex flex-col gap-4 rounded-xl border p-6 md:flex-row md:items-center md:justify-between md:p-8">
  <div class="flex flex-col gap-1">
    <h2 class="text-xl font-semibold tracking-tight">Prefer email?</h2>
    <p class="text-muted-foreground">
      Drop me a line at
      <a href={mailto} class="text-primary transition-opacity hover:opacity-80">
        {site.contactEmail}
      </a>
      and I'll take it from there.
    </p>
  </div>
  <Button class="w-fit shrink-0" href={mailto}>
    <Icon icon="fa6-solid:envelope" class="size-3.5" />
    Send an email
  </Button>
</section>
