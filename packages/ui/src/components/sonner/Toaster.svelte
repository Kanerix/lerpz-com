<script lang="ts">
import { Toaster as SonnerToaster } from "svelte-sonner";

let { ...rest } = $props();
</script>

<SonnerToaster richColors {...rest} />
