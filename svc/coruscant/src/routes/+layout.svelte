<script lang="ts">
import { Toaster } from "@lerpz/ui/components/sonner";
import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
import { ModeWatcher } from "mode-watcher";
import { msalStore } from "$lib/auth/msal.svelte.js";
import "../app.css";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();

const queryClient = new QueryClient({
    defaultOptions: { queries: { staleTime: 60 * 1000 } },
});

$effect(() => {
    msalStore.initialize();
});
</script>

<ModeWatcher />
<QueryClientProvider client={queryClient}>
  {@render children()}
</QueryClientProvider>
<Toaster />
