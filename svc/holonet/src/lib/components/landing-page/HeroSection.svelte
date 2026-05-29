<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { TextLoop } from "@lerpz/ui/components/text-loop";
import { msalStore } from "$lib/auth/msal.svelte.js";
</script>

<section class="flex flex-col gap-6 py-16 md:py-24">
  <p class="text-sm font-medium text-muted-foreground uppercase tracking-widest">Lerpz AI Portal</p>

  <h1 class="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl xl:text-7xl max-w-3xl">
    AI tools for{" "}
    <span class="inline-flex h-[1.15em] items-end text-primary">
      <TextLoop interval={3} items={["your daily work.", "your whole team.", "your organisation."]} />
    </span>
  </h1>

  <p class="max-w-xl text-lg text-muted-foreground leading-relaxed">
    Sign in with your work account to access AI chat, image generation, and more —
    centralised and maintained for everyone in the organisation.
  </p>

  <div class="flex items-center gap-3 pt-2">
    <Button size="lg" onclick={() => msalStore.loginRedirect()}>
      Sign in
      <Icon icon="mdi:arrow-right" class="size-4" />
    </Button>
    <Button variant="ghost" size="lg" href="/docs">Documentation</Button>
  </div>
</section>
