<script lang="ts">
import type { Snippet } from "svelte";
import Footer from "$lib/components/landing-page/Footer.svelte";
import Header from "$lib/components/landing-page/Header.svelte";

let { children }: { children: Snippet } = $props();
</script>

<div class="relative flex min-h-screen flex-col">
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 -z-10 flex items-start justify-center"
  >
    <div class="mt-16 h-150 w-225 rounded-full dark:bg-primary/10 blur-3xl"></div>
  </div>
  <Header />

  <main class="mx-auto w-full max-w-5xl flex-1 px-4 py-12 md:py-16">
    {@render children()}
  </main>

  <Footer />
</div>
