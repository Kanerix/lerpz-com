<script lang="ts">
import { SidebarInset, SidebarProvider } from "@lerpz/ui/components/sidebar";
import type { Snippet } from "svelte";
import AppSidebar from "./AppSidebar.svelte";
import AppTopbar from "./AppTopbar.svelte";

let { children }: { children: Snippet } = $props();
</script>

<SidebarProvider>
  <AppSidebar />
  <SidebarInset>
    <AppTopbar />
    {@render children()}
  </SidebarInset>
</SidebarProvider>
