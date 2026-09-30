<script lang="ts">
import type { Snippet } from "svelte";
import { msalStore } from "$lib/auth/msal.svelte.js";

let { children }: { children?: Snippet } = $props();
</script>

{#key msalStore.accountKey}
  {@render children?.()}
{/key}
