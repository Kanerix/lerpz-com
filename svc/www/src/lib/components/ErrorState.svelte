<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { resolve } from "$app/paths";

let {
    status = 404,
    title = "Page not found",
    description = "The page you're looking for doesn't exist or has been moved.",
}: {
    status?: number;
    title?: string;
    description?: string;
} = $props();
</script>

<section class="flex flex-col items-center gap-6 py-16 text-center md:py-24">
  <div class="flex max-w-md flex-col items-center gap-2">
    <h1 class="text-7xl font-bold tracking-tighter text-foreground">{status}</h1>
    <h2 class="text-xl font-semibold tracking-tight text-foreground">{title}</h2>
    <p class="text-sm text-muted-foreground">{description}</p>
  </div>

  <div class="flex items-center gap-3">
    <Button href={resolve("/")} size="lg">
      Go home
      <Icon icon="fa6-solid:arrow-right" class="size-4" />
    </Button>
    <Button variant="outline" href={resolve("/contact")} size="lg">Contact me</Button>
  </div>
</section>
