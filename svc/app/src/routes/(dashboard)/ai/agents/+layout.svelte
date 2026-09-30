<script lang="ts">
import type { Snippet } from "svelte";
import { agentAccountKey } from "$lib/http/agent-mutator.js";

let { children }: { children?: Snippet } = $props();
</script>

{#key agentAccountKey()}
  {@render children?.()}
{/key}
