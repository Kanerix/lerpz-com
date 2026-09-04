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

interface AdminSection {
    title: string;
    description: string;
    icon: string;
    href: string;
}

const sections: AdminSection[] = [
    {
        title: "Users",
        description: "Manage accounts, roles and access.",
        icon: "fa6-solid:users",
        href: "/admin/users",
    },
    {
        title: "Content",
        description: "Review and moderate generated content.",
        icon: "fa6-regular:folder-open",
        href: "/admin/content",
    },
    {
        title: "System",
        description: "Inspect service health and configuration.",
        icon: "fa6-solid:server",
        href: "/admin/system",
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
      {#each sections as section (section.href)}
        <a href={section.href} class="outline-none focus-visible:ring-2 focus-visible:ring-ring rounded-xl">
          <Card class="flex h-full flex-col transition-colors hover:border-primary/50 hover:bg-accent/40">
            <CardHeader>
              <div class="flex items-center gap-2">
                <Icon icon={section.icon} class="size-4 text-muted-foreground" />
                <CardTitle>{section.title}</CardTitle>
              </div>
              <CardDescription>{section.description}</CardDescription>
            </CardHeader>
            <CardContent class="mt-auto text-sm text-muted-foreground">
              <span class="inline-flex items-center gap-1.5">
                Open
                <Icon icon="fa6-solid:arrow-right" class="size-3" />
              </span>
            </CardContent>
          </Card>
        </a>
      {/each}
    </div>
  </div>
  </ScrollArea>
{/if}
