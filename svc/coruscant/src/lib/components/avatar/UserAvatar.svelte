<script lang="ts">
import Icon from "@iconify/svelte";
import {
    Avatar,
    AvatarFallback,
    AvatarImage,
} from "@lerpz/ui/components/avatar";
import { cn } from "@lerpz/ui/lib/utils";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { getCurrentUserPhotoUrl } from "$lib/graph/photo.js";

let {
    size = "sm",
    class: className,
}: {
    size?: "sm" | "default" | "lg";
    class?: string;
} = $props();

const account = $derived(msalStore.activeAccount);

let photo = $state<string | undefined>(undefined);
$effect(() => {
    const accountId = account?.homeAccountId;
    if (!accountId) {
        photo = undefined;
        return;
    }
    getCurrentUserPhotoUrl().then((url) => {
        photo = url ?? undefined;
    });
});

const initials = $derived(
    account?.name
        ?.split(" ")
        .map((n) => n?.[0]?.toUpperCase() ?? "")
        .filter(Boolean)
        .slice(0, 2)
        .join("") || "",
);
</script>

<Avatar {size} class={cn("shrink-0", className)}>
  <AvatarImage src={photo ?? ""} alt={account?.name ?? "You"} />
  <AvatarFallback>
    {#if initials}
      <span class="text-xs font-medium">{initials}</span>
    {:else}
      <Icon icon="fa6-solid:user" class="size-3.5" />
    {/if}
  </AvatarFallback>
</Avatar>
