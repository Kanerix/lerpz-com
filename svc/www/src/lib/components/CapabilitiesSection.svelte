<script lang="ts">
import Icon from "@iconify/svelte";

const capabilities = [
    {
        icon: "fa6-regular:comment",
        title: "Conversations",
        description:
            "Chat that streams as it writes. Your history sticks around and stays searchable.",
    },
    {
        icon: "fa6-regular:image",
        title: "Generation",
        description:
            "Images and short video from a prompt. They land in a gallery, not your downloads folder.",
    },
    {
        icon: "fa6-solid:robot",
        title: "Agents that remember",
        description:
            "Agents keep their memory between runs, so they pick up where they stopped.",
    },
    {
        icon: "fa6-solid:sliders",
        title: "Central control",
        description:
            "Decide once who gets which models. No arguing about it in every repo.",
    },
];
</script>

<section id="platform" class="flex scroll-mt-24 flex-col gap-8 py-16 md:py-24">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">What the platform does</h2>
    <p class="leading-relaxed text-muted-foreground">
      It's four tools, really. They share one login and one set of models.
    </p>
  </div>

  <div class="grid gap-x-12 gap-y-2 sm:grid-cols-2">
    {#each capabilities as capability (capability.title)}
      <div class="flex gap-4 border-t border-border/60 py-5">
        <Icon icon={capability.icon} class="mt-0.5 size-4.5 shrink-0 text-primary" />
        <div class="flex flex-col gap-1.5">
          <h3 class="text-sm font-semibold">{capability.title}</h3>
          <p class="text-sm leading-relaxed text-muted-foreground">{capability.description}</p>
        </div>
      </div>
    {/each}
  </div>
</section>
