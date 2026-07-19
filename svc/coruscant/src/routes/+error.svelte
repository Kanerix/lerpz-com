<script lang="ts">
import { page } from "$app/state";
import { Button } from "@lerpz/ui/components/button";

const isNotFound = $derived(page.status === 404);
const title = $derived(isNotFound ? "Page not found" : "Something went wrong");
const description = $derived(
  isNotFound
    ? "The page you're looking for doesn't exist or has been moved."
    : (page.error?.message ?? "An unexpected error occurred. Please try again later."),
);
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-background px-4">
  <div class="flex flex-col items-center gap-6 text-center max-w-md">
    <img src="/lerpz.svg" alt="Lerpz" class="h-12 w-12" />
    <div class="flex flex-col items-center gap-2">
      <h1 class="text-7xl font-bold tracking-tighter text-foreground">{page.status}</h1>
      <h2 class="text-xl font-semibold tracking-tight text-foreground">{title}</h2>
      <p class="text-sm text-muted-foreground">
        {description}
      </p>
    </div>
    <div class="flex items-center gap-3">
      <Button href="/" size="lg">Go home</Button>
      <Button variant="outline" href="/login" size="lg">Sign in</Button>
    </div>
  </div>
</div>
