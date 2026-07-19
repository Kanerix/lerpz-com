<script lang="ts">
import Icon from "@iconify/svelte";
import { Input } from "@lerpz/ui/components/input";
import { SidebarTrigger } from "@lerpz/ui/components/sidebar";
import ThemeButton from "$lib/components/ThemeButton.svelte";
import NotificationBell from "./NotificationBell.svelte";

let search = $state("");
</script>

<header class="flex items-center shrink-0 gap-2 border-b px-4 h-12">
  <div class="flex w-[--sidebar-width-icon] shrink-0 items-center justify-center">
    <SidebarTrigger />
  </div>
  <div class="grow"></div>
  <div class="relative w-full max-w-xs">
    <Icon
      icon="fa6-solid:magnifying-glass"
      class="pointer-events-none absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground"
    />
    <Input
      value={search}
      oninput={(e) => (search = e.currentTarget.value)}
      placeholder="Search…"
      aria-label="Search"
      class="pl-9"
    />
  </div>
  <div class="grow"></div>
  <div class="flex items-center gap-1">
    <NotificationBell />
    <ThemeButton />
  </div>
</header>
