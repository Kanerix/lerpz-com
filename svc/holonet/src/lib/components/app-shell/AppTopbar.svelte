<script lang="ts">
import { SidebarTrigger } from "@lerpz/ui/components/sidebar";
import ThemeButton from "$lib/components/ThemeButton.svelte";
</script>

<header class="flex items-center shrink-0 gap-2 border-b px-4 h-12">
  <div class="grow">
    <SidebarTrigger class="ml-1" />
  </div>
  <div>
    <ThemeButton />
  </div>
</header>
