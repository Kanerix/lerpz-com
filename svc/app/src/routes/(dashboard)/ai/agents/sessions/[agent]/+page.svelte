<script lang="ts">
import { agentAccountKey } from "$lib/http/agent-mutator.js";
import type { PageProps } from "./$types.js";
import RuntimeDetails from "./RuntimeDetails.svelte";

let { params }: PageProps = $props();
</script>

{#key `${agentAccountKey()}:${params.agent}`}
  <RuntimeDetails agent={params.agent} />
{/key}
