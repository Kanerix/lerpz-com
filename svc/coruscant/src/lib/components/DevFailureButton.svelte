<script lang="ts">
// TEMPORARY: dev-only trigger that hits the backend's failure endpoint to
// exercise the ErrorDialog. Remove this component (and its usage in the root
// layout) once error handling is verified.
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { ErrorDialog } from "$lib/components/error-dialog";
import { authenticatedFetch } from "$lib/http/fetch.js";

let open = $state(false);
let error = $state<unknown>(null);
let loading = $state(false);

async function triggerFailure() {
    loading = true;
    error = null;
    try {
        // Roughly half the time, blow up with a genuine runtime JS error
        // (real stack trace) to exercise the ErrorDialog's traceback fallback
        // instead of the problem+json path.
        if (Math.random() < 0.5) {
            const boom = undefined as unknown as { explode(): void };
            boom.explode();
        }
        const res = await authenticatedFetch("/api/v1/failure");
        if (res.ok) {
            error = new Error(
                `Expected a failure, but /api/v1/failure returned ${res.status}.`,
            );
            open = true;
            return;
        }
        try {
            error = await res.json();
        } catch {
            const body = await res.text().catch(() => res.statusText);
            error = new Error(`HTTP ${res.status}: ${body || res.statusText}`);
        }
        open = true;
    } catch (err) {
        error = err;
        open = true;
    } finally {
        loading = false;
    }
}
</script>

<Button
  variant="destructive"
  size="sm"
  onclick={triggerFailure}
  disabled={loading}
  class="fixed bottom-4 right-4 z-40 shadow-lg 2xl:flex hidden"
  title="Temporary: trigger /api/v1/failure"
>
  <Icon
    icon={loading ? "fa6-solid:spinner" : "fa6-solid:bug"}
    class={loading ? "size-3.5 animate-spin" : "size-3.5"}
  />
  Trigger failure
</Button>

<ErrorDialog bind:open {error} />
