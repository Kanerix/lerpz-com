<script lang="ts">
import Icon from "@iconify/svelte";
import {
    Avatar,
    AvatarFallback,
    AvatarImage,
} from "@lerpz/ui/components/avatar";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import {
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    useSidebar,
} from "@lerpz/ui/components/sidebar";
import { msalStore } from "$lib/auth/msal.svelte.js";
import { getCurrentUserPhotoUrl } from "$lib/graph/photo.js";

const sidebar = useSidebar();
const account = $derived(msalStore.activeAccount);

let avatarSrc = $state<string | undefined>(undefined);

$effect(() => {
    const accountId = account?.homeAccountId;
    if (!accountId) {
        avatarSrc = undefined;
        return;
    }
    getCurrentUserPhotoUrl().then((url) => {
        avatarSrc = url ?? undefined;
    });
});

const avatarFallback = $derived(
    account?.name
        ?.split(" ")
        .map((n) => n?.[0]?.toUpperCase() ?? "")
        .filter(Boolean)
        .slice(0, 2)
        .join("") || "?",
);
</script>

<SidebarMenu>
  <SidebarMenuItem>
    <DropdownMenu>
      <DropdownMenuTrigger>
        <SidebarMenuButton size="lg">
          <Avatar class="h-8 w-8 rounded-lg">
            <AvatarImage src={avatarSrc ?? ""} alt={account?.name ?? ""} />
            <AvatarFallback class="rounded-lg">{avatarFallback}</AvatarFallback>
          </Avatar>
          <div class="grid flex-1 text-left text-sm leading-tight">
            <span class="truncate font-medium">{account?.name}</span>
            <span class="truncate text-xs">{account?.username}</span>
          </div>
          <Icon icon="mdi:unfold-more-horizontal" class="ml-auto size-4" />
        </SidebarMenuButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side={sidebar.isMobile ? "bottom" : "right"}
        align="end"
        sideOffset={4}
      >
        <DropdownMenuGroup>
          <DropdownMenuLabel class="p-0 font-normal">
            <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
              <Avatar class="h-8 w-8 rounded-lg">
                <AvatarImage src={avatarSrc ?? ""} alt={account?.name ?? ""} />
                <AvatarFallback class="rounded-lg">{avatarFallback}</AvatarFallback>
              </Avatar>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-medium">{account?.name}</span>
                <span class="truncate text-xs">{account?.username}</span>
              </div>
            </div>
          </DropdownMenuLabel>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem value="upgrade">
            <Icon icon="mdi:auto-awesome" />
            Upgrade to Pro
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem value="account"><Icon icon="mdi:verified" />Account</DropdownMenuItem>
          <DropdownMenuItem value="notifications"><Icon icon="mdi:bell-outline" />Notifications</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuItem value="switch" onclick={() => msalStore.switchAccount()}>
          <Icon icon="mdi:account-switch-outline" />
          Switch Account
        </DropdownMenuItem>
        <DropdownMenuItem value="logout" onclick={() => msalStore.logoutRedirect()}>
          <Icon icon="mdi:logout" />
          Log out
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </SidebarMenuItem>
</SidebarMenu>
