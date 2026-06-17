<script lang="ts">
import { Button } from "@lerpz/ui/components/button";
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-background px-4">
  <div class="flex flex-col items-center gap-6 text-center max-w-md">
    <img src="/lerpz.svg" alt="Lerpz" class="h-12 w-12" />
    <div class="flex flex-col items-center gap-2">
      <h1 class="text-7xl font-bold tracking-tighter text-foreground">404</h1>
      <h2 class="text-xl font-semibold tracking-tight text-foreground">Page not found</h2>
      <p class="text-sm text-muted-foreground">
        The page you're looking for doesn't exist or has been moved.
      </p>
    </div>
    <div class="flex items-center gap-3">
      <Button href="/" size="lg">Go home</Button>
      <Button variant="outline" href="/login" size="lg">Sign in</Button>
    </div>
  </div>
</div>
