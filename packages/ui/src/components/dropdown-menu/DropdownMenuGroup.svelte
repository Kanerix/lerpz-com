<script lang="ts">
import { Menu } from "@ark-ui/svelte/menu";
import type { Snippet } from "svelte";

let { children }: { children?: Snippet } = $props();
</script>

<Menu.ItemGroup data-slot="dropdown-menu-group">{@render children?.()}</Menu.ItemGroup>
