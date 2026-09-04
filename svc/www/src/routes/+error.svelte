<script lang="ts">
import { page } from "$app/state";
import ErrorState from "$lib/components/ErrorState.svelte";

const isNotFound = $derived(page.status === 404);
const title = $derived(isNotFound ? "Page not found" : "Something went wrong");
const description = $derived(
    isNotFound
        ? "The page you're looking for doesn't exist or has been moved."
        : (page.error?.message ??
              "An unexpected error occurred. Please try again later."),
);
</script>

<svelte:head>
  <title>Lerpz – {title}</title>
</svelte:head>

<ErrorState status={page.status} {title} {description} />
