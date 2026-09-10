<script lang="ts">
import Icon from "@iconify/svelte";
import { Badge } from "@lerpz/ui/components/badge";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@lerpz/ui/components/card";
import { ScrollArea } from "@lerpz/ui/components/scroll-area";
import { goto } from "$app/navigation";
import { msalStore } from "$lib/auth/msal.svelte.js";

// Guard the route itself: hiding the sidebar entry isn't enough since a user
// could navigate here directly. Anyone without the Lerpz.Admin role is bounced
// back to the dashboard root.
const isAdmin = $derived(msalStore.hasRole("Lerpz.Admin"));

$effect(() => {
    if (!isAdmin) {
        goto("/");
    }
});

type AdminSection = {
    title: string;
    description: string;
    icon: string;
};

const sections: AdminSection[] = [
    {
        title: "Users",
        description: "Manage accounts, roles and access.",
        icon: "fa6-solid:users",
    },
    {
        title: "Content",
        description: "Review and moderate generated content.",
        icon: "fa6-regular:folder-open",
    },
    {
        title: "System",
        description: "Inspect service health and configuration.",
        icon: "fa6-solid:server",
    },
];
</script>

{#if isAdmin}
  <ScrollArea class="h-full" orientation="vertical">
  <div class="mx-auto w-full max-w-5xl space-y-6 p-6">
    <header class="flex items-center gap-3">
      <Icon icon="fa6-solid:shield-halved" class="size-6 text-primary" />
      <div class="flex flex-1 items-center gap-2">
        <h1 class="text-2xl font-semibold tracking-tight">Admin</h1>
        <Badge variant="secondary">Lerpz.Admin</Badge>
      </div>
    </header>

    <p class="text-sm text-muted-foreground">
      Signed in as <span class="font-medium text-foreground">{msalStore.activeAccount?.name}</span>.
      These tools are only visible to accounts with the Lerpz.Admin role.
    </p>

    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each sections as section (section.title)}
        <Card class="flex h-full flex-col">
          <CardHeader>
            <div class="flex items-center gap-2">
              <Icon icon={section.icon} class="size-4 text-muted-foreground" />
              <CardTitle>{section.title}</CardTitle>
            </div>
            <CardDescription>{section.description}</CardDescription>
          </CardHeader>
          <CardContent class="mt-auto text-sm text-muted-foreground">Not built yet.</CardContent>
        </Card>
      {/each}
    </div>
  </div>
  </ScrollArea>
{/if}
