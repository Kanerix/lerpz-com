<script lang="ts">
import { ModeWatcher } from "mode-watcher";
import Footer from "$lib/components/Footer.svelte";
import Header from "$lib/components/Header.svelte";
import StatusNotice from "$lib/components/StatusNotice.svelte";
import "../app.css";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />

<div class="relative flex min-h-screen flex-col">
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 -z-10 flex items-start justify-center"
  >
    <div class="mt-16 h-150 w-225 rounded-full dark:bg-primary/10 blur-3xl"></div>
  </div>
  <StatusNotice />
  <Header />

  <main class="mx-auto w-full max-w-5xl flex-1 px-4 py-12 md:py-16">
    {@render children()}
  </main>

  <Footer />
</div>
