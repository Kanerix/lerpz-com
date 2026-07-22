<script lang="ts">
import Icon from "@iconify/svelte";

const values = [
    {
        icon: "fa6-solid:layer-group",
        title: "One shared toolkit",
        description:
            "Every team works from the same centrally maintained AI portal, so tooling stays consistent across the organisation.",
    },
    {
        icon: "fa6-solid:shield-halved",
        title: "Secure by default",
        description:
            "Access is authenticated through your organisation's identity provider, and we only process the data we genuinely need.",
    },
    {
        icon: "fa6-solid:gauge-high",
        title: "Built to move fast",
        description:
            "We ship improvements continuously so the platform keeps pace with how your teams actually work.",
    },
];
</script>

<svelte:head>
  <title>About — Lerpz AI</title>
  <meta name="description" content="Learn about Lerpz AI, the centrally maintained AI portal for your organisation." />
</svelte:head>

<section class="flex flex-col gap-4 py-8 md:py-12">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">About</p>
  <h1 class="max-w-2xl text-4xl font-semibold tracking-tight md:text-5xl">
    The AI portal for your organisation.
  </h1>
  <p class="max-w-2xl text-lg leading-relaxed text-muted-foreground">
    Lerpz AI brings your organisation's AI tooling together in one place. Instead of every
    team stitching together their own set of tools, Lerpz maintains a single, well-supported
    platform so everyone works from the same foundation.
  </p>
</section>

<section class="flex flex-col gap-6 py-8 md:py-12">
  <div class="flex flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight">What we care about</h2>
    <p class="text-muted-foreground">The principles that guide how we build the platform.</p>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each values as value (value.title)}
      <div class="flex flex-col gap-3 rounded-xl border p-6">
        <div class="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
          <Icon icon={value.icon} class="size-5" />
        </div>
        <h3 class="text-base font-semibold">{value.title}</h3>
        <p class="text-sm leading-relaxed text-muted-foreground">{value.description}</p>
      </div>
    {/each}
  </div>
</section>

<section class="flex flex-col gap-4 rounded-xl border p-6 md:p-8">
  <h2 class="text-2xl font-semibold tracking-tight">Get in touch</h2>
  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    Want to know more about Lerpz AI or how it fits your organisation? We're happy to talk.
  </p>
  <a
    href="/contact"
    class="inline-flex w-fit items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90"
  >
    Contact us
    <Icon icon="fa6-solid:arrow-right" class="size-3.5" />
  </a>
</section>
