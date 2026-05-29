<script lang="ts">
import { Select } from "@ark-ui/svelte/select";

let { placeholder = "" }: { placeholder?: string } = $props();
</script>

<Select.ValueText data-slot="select-value" {placeholder} />
