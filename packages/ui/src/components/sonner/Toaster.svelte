<script lang="ts">
import { mode } from "mode-watcher";
import { Toaster as SonnerToaster } from "svelte-sonner";

let { ...rest } = $props();
</script>

<SonnerToaster theme={$mode} richColors {...rest} />
