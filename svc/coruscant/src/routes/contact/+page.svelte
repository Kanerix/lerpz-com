<script lang="ts">
import Icon from "@iconify/svelte";
import Footer from "$lib/components/landing-page/Footer.svelte";
import Header from "$lib/components/landing-page/Header.svelte";

const contactEmail = "kas@lerpz.com";

const channels = [
    {
        icon: "fa6-solid:envelope",
        title: "Email us",
        description: "The fastest way to reach the Lerpz team for questions or support.",
        actionLabel: contactEmail,
        href: `mailto:${contactEmail}`,
    },
    {
        icon: "fa6-brands:github",
        title: "GitHub",
        description: "Browse our projects, report issues, or follow along with development.",
        actionLabel: "github.com/lerpz",
        href: "https://github.com/lerpz",
    },
];
</script>

<svelte:head>
  <title>Contact — Lerpz AI</title>
  <meta name="description" content="Get in touch with the Lerpz AI team." />
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
    <section class="flex flex-col gap-4 py-8 md:py-12">
      <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Contact</p>
      <h1 class="max-w-2xl text-4xl font-semibold tracking-tight md:text-5xl">
        Let's talk.
      </h1>
      <p class="max-w-2xl text-lg leading-relaxed text-muted-foreground">
        Have a question, a request, or feedback about Lerpz AI? Reach out and we'll get back to
        you as soon as we can.
      </p>
    </section>

    <section class="grid grid-cols-1 gap-4 py-8 md:grid-cols-2 md:py-12">
      {#each channels as channel (channel.title)}
        <div class="flex flex-col gap-3 rounded-xl border p-6">
          <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
            <Icon icon={channel.icon} class="size-5" />
          </div>
          <h2 class="text-base font-semibold">{channel.title}</h2>
          <p class="text-sm leading-relaxed text-muted-foreground">{channel.description}</p>
          <a
            href={channel.href}
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
          Drop us a line at
          <a href={`mailto:${contactEmail}`} class="text-primary transition-opacity hover:opacity-80">
            {contactEmail}
          </a>
          and we'll take it from there.
        </p>
      </div>
      <a
        href={`mailto:${contactEmail}`}
        class="inline-flex w-fit items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90"
      >
        <Icon icon="fa6-solid:envelope" class="size-3.5" />
        Send an email
      </a>
    </section>
  </main>

  <Footer />
</div>
