<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { resolve } from "$app/paths";
import { site } from "$lib/config.js";
</script>

<section class="flex flex-col gap-6 py-16 md:py-20">
  <div class="flex max-w-2xl flex-col gap-3">
    <h2 class="text-3xl font-semibold tracking-tight md:text-4xl">
      Have a look for yourself.
    </h2>
    <p class="text-lg leading-relaxed text-muted-foreground">
      Sign in with your work account to start using the platform, or send a
      message if you'd like to talk through how it would fit your organisation.
    </p>
  </div>

  <div class="flex flex-wrap items-center gap-3">
    <Button size="lg" href={site.appUrl}>
      Open the app
      <Icon icon="fa6-solid:arrow-right" class="size-4" />
    </Button>
    <Button variant="outline" size="lg" href={resolve("/contact")}>Get in touch</Button>
  </div>
</section>
