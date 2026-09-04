<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { site } from "$lib/config.js";
</script>

<section class="flex flex-col gap-5 rounded-xl border bg-card/40 p-6 md:p-8">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Behind Lerpz</p>

  <h2 class="max-w-2xl text-2xl font-semibold tracking-tight md:text-3xl">
    Lerpz AI is one of my projects.
  </h2>

  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    The platform grew out of a habit of building things properly and seeing them
    through. The portfolio collects the rest of that work — other projects,
    experiments, and the engineering behind them.
  </p>

  <div class="flex flex-wrap items-center gap-3 pt-1">
    <Button href={site.portfolioUrl} target="_blank" rel="noopener noreferrer">
      View the portfolio
      <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-3.5" />
    </Button>
    <Button
      variant="outline"
      href={site.githubUrl}
      target="_blank"
      rel="noopener noreferrer"
    >
      <Icon icon="fa6-brands:github" class="size-4" />
      GitHub
    </Button>
  </div>
</section>
