<script lang="ts">
import Icon from "@iconify/svelte";
import { resolve } from "$app/paths";
</script>

<div class="border-b border-primary/25 bg-primary/10">
  <div class="mx-auto flex max-w-5xl items-start gap-3 px-4 py-3">
    <Icon
      icon="fa6-solid:plug-circle-exclamation"
      class="mt-0.5 size-4 shrink-0 text-primary"
      aria-hidden="true"
    />
    <p class="text-sm leading-relaxed text-foreground">
      <span class="font-semibold">The app is probably switched off.</span>
      Running a Kubernetes cluster around the clock isn't cheap, and I'm the one
      paying for it, so I shut it down when nobody's using it. If the app won't
      load, that's why.
      <a
        href={resolve("/contact")}
        class="font-medium text-primary underline underline-offset-4 transition-opacity hover:opacity-80"
      >
        Give me a shout
      </a>
      and I'll turn it back on.
    </p>
  </div>
</div>
