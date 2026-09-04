<script lang="ts">
import ErrorState from "$lib/components/ErrorState.svelte";
</script>

<!--
  Prerendering only writes files for successful responses, so `+error.svelte` on
  its own never produces a `404.html`. This route renders the same not-found
  screen at `/404`, which the prerenderer writes out as `build/404.html` — the
  file static hosts (nginx, GitHub Pages, Azure Static Web Apps) serve for
  unknown paths.
-->

<svelte:head>
  <title>Page not found — Lerpz AI</title>
  <meta name="robots" content="noindex" />
</svelte:head>

<ErrorState />
