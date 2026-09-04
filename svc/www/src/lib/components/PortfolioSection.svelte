<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { site } from "$lib/config.js";
</script>

<section class="flex flex-col gap-5 rounded-xl border bg-card/40 p-6 md:p-8">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Behind Lerpz</p>

  <h2 class="max-w-2xl text-2xl font-semibold tracking-tight md:text-3xl">
    There's no team. It's just me.
  </h2>

  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    I wrote all of this. Every service, the infrastructure it runs on, and this
    site. When something breaks at two in the morning, I'm the one fixing it.
  </p>

  <p class="max-w-2xl leading-relaxed text-muted-foreground">
    If you want the official version, my CV has the jobs, the education and
    where I've worked.
  </p>

  <div class="flex flex-wrap items-center gap-3 pt-1">
    <Button href={site.portfolioUrl} target="_blank" rel="noopener noreferrer">
      View my CV
      <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-3.5" />
    </Button>
    <Button variant="outline" href={site.githubUrl} target="_blank" rel="noopener noreferrer">
      <Icon icon="fa6-brands:github" class="size-4" />
      See the code
    </Button>
  </div>
</section>
