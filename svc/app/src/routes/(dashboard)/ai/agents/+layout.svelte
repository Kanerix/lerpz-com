<script lang="ts">
import type { Snippet } from "svelte";
import { msalStore } from "$lib/auth/msal.svelte.js";

let { children }: { children?: Snippet } = $props();
const accountKey = $derived(msalStore.accountKey);
</script>

{#key accountKey}
  {@render children?.()}
{/key}
