<script lang="ts">
import { msalStore } from "$lib/auth/msal.svelte.js";
import type { PageProps } from "./$types.js";
import RuntimeDetails from "./RuntimeDetails.svelte";

let { params }: PageProps = $props();
</script>

{#key `${msalStore.accountKey}:${params.agent}`}
  <RuntimeDetails agent={params.agent} />
{/key}
