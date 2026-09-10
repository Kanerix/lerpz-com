<script lang="ts">
import { Toaster } from "@lerpz/ui/components/sonner";
import {
    MutationCache,
    QueryCache,
    QueryClient,
    QueryClientProvider,
} from "@tanstack/svelte-query";
import { ModeWatcher } from "mode-watcher";
import { msalStore } from "$lib/auth/msal.svelte.js";
import {
    ErrorDialog,
    errorDialog,
    showError,
} from "$lib/components/error-dialog/index.js";
import "../app.css";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();

// Every REST call ultimately flows through the query client, so this is the one
// place error handling lives: a failed *initial* response opens the shared
// error dialog. Direct (non-query) calls opt in with the same `showError`
// helper, and streams keep their own in-stream handling.
const queryClient = new QueryClient({
    defaultOptions: { queries: { staleTime: 60 * 1000 } },
    queryCache: new QueryCache({
        onError: (error, query) => {
            if (query.meta?.skipGlobalErrorDialog) return;
            // Only surface the dialog while there is no data yet (the initial
            // response). A failed background refetch shouldn't interrupt a view
            // that already has something to show.
            if (query.state.data !== undefined) return;
            showError(error);
        },
    }),
    mutationCache: new MutationCache({
        onError: (error, _variables, _context, mutation) => {
            if (mutation.meta?.skipGlobalErrorDialog) return;
            showError(error);
        },
    }),
});

$effect(() => {
    msalStore.initialize();
});
</script>

<ModeWatcher />
<QueryClientProvider client={queryClient}>
  {@render children()}
</QueryClientProvider>
<ErrorDialog bind:open={errorDialog.open} error={errorDialog.error} />
<Toaster />
