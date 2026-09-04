<script lang="ts">
import { InteractionStatus } from "@azure/msal-browser";
import type { Snippet } from "svelte";
import { goto } from "$app/navigation";
import { msalStore } from "$lib/auth/msal.svelte.js";
import AppShell from "$lib/components/app-shell/AppShell.svelte";
import LoadingPage from "$lib/components/LoadingPage.svelte";

let { children }: { children: Snippet } = $props();

$effect(() => {
    if (
        msalStore.inProgress === InteractionStatus.None &&
        msalStore.accounts.length === 0
    ) {
        goto("/login");
    }
});
</script>

{#if msalStore.inProgress !== InteractionStatus.None}
  <LoadingPage />
{:else if msalStore.accounts.length === 0}
  <LoadingPage />
{:else}
  <div class="overflow-hidden h-screen w-screen">
    <AppShell>
      <main class="w-full flex-1 min-h-0 overflow-hidden">
        {@render children()}
      </main>
    </AppShell>
  </div>
{/if}
