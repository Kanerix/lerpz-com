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
      <span class="font-semibold">The app is probably having a lie-in.</span>
      Keeping a Kubernetes cluster awake around the clock costs real money, and I
      am one person with exactly one bank account — so the live app is not always
      switched on. If it refuses to load, that is my wallet talking, not you.
      <a
        href={resolve("/contact")}
        class="font-medium text-primary underline underline-offset-4 transition-opacity hover:opacity-80"
      >
        Give me a shout
      </a>
      and I will gladly wake it up.
    </p>
  </div>
</div>
