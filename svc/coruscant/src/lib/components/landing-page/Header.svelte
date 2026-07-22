<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@lerpz/ui/components/dropdown-menu";
import { cn } from "@lerpz/ui/lib/utils";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { msalStore } from "$lib/auth/msal.svelte.js";
import UserAvatar from "$lib/components/avatar/UserAvatar.svelte";
import ThemeButton from "$lib/components/ThemeButton.svelte";

const navLinks = [
    { label: "Tools", href: "/ai" },
    { label: "Docs", href: "/docs" },
    { label: "Changelog", href: "/changelog" },
    { label: "Status", href: "/status" },
];

const isAuthenticated = $derived(msalStore.isAuthenticated);
const account = $derived(msalStore.activeAccount);
const pathname = $derived(page.url.pathname);

function isActive(href: string): boolean {
    return pathname === href || pathname.startsWith(`${href}/`);
}

let mobileOpen = $state(false);

let signingIn = $state(false);

async function signIn() {
    signingIn = true;
    try {
        await msalStore.loginRedirect();
    } finally {
        signingIn = false;
    }
}

// Collapse the mobile menu whenever navigation changes the current route.
$effect(() => {
    void pathname;
    mobileOpen = false;
});
</script>

<header class="sticky top-0 z-50 border-b border-border/60 bg-background/80 backdrop-blur-md">
  <div class="mx-auto flex h-16 max-w-[1024px] items-center justify-between gap-4 px-4">
    <a
      href="/"
      class="flex shrink-0 items-center gap-2.5 font-semibold tracking-tight text-foreground transition-opacity hover:opacity-80"
    >
      <img src="/lerpz.svg" alt="Lerpz Logo" class="h-7 w-7" />
    </a>

    <nav class="hidden items-center gap-1 md:flex">
      {#each navLinks as link}
        {@const active = isActive(link.href)}
        <a
          href={link.href}
          aria-current={active ? "page" : undefined}
          class={cn(
            "rounded-md px-3 py-1.5 text-sm font-medium transition-colors",
            active
              ? "bg-accent text-foreground"
              : "text-muted-foreground hover:bg-accent/60 hover:text-foreground",
          )}
        >
          {link.label}
        </a>
      {/each}
    </nav>

    <div class="flex items-center gap-1.5">
      <ThemeButton />

      {#if isAuthenticated}
        <div class="hidden items-center sm:flex">
          <DropdownMenu align="end" sideOffset={8}>
            <DropdownMenuTrigger
              aria-label="Account menu"
              class="flex items-center rounded-full outline-none ring-offset-background transition hover:opacity-80 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              <UserAvatar size="default" />
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-60">
              <DropdownMenuLabel class="p-0 font-normal">
                <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                  <UserAvatar size="default" />
                  <div class="grid flex-1 leading-tight">
                    <span class="truncate font-medium">{account?.name}</span>
                    <span class="truncate text-xs text-muted-foreground">{account?.username}</span>
                  </div>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem value="dashboard" onclick={() => goto("/ai/chats")}>
                <Icon icon="fa6-solid:table-columns" />
                Dashboard
              </DropdownMenuItem>
              <DropdownMenuItem value="switch" onclick={() => msalStore.switchAccount()}>
                <Icon icon="fa6-solid:arrow-right-arrow-left" />
                Switch account
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                value="logout"
                onclick={() => msalStore.logoutRedirect()}
                class="text-destructive focus:text-destructive hover:text-destructive"
              >
                <Icon icon="fa6-solid:right-from-bracket" />
                Sign out
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      {:else}
        <Button class="hidden sm:inline-flex" disabled={signingIn} onclick={signIn}>
          {#if signingIn}
            <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
          {/if}
          Sign in
        </Button>
      {/if}

      <Button
        variant="ghost"
        size="icon"
        class="md:hidden"
        aria-label={mobileOpen ? "Close menu" : "Open menu"}
        aria-expanded={mobileOpen}
        onclick={() => (mobileOpen = !mobileOpen)}
      >
        <Icon icon={mobileOpen ? "fa6-solid:xmark" : "fa6-solid:bars"} class="size-5" />
      </Button>
    </div>
  </div>

  {#if mobileOpen}
    <div class="border-t border-border/60 bg-background/95 backdrop-blur-md md:hidden">
      <nav class="mx-auto flex max-w-[1024px] flex-col gap-1 px-4 py-3">
        {#each navLinks as link}
          {@const active = isActive(link.href)}
          <a
            href={link.href}
            aria-current={active ? "page" : undefined}
            class={cn(
              "rounded-md px-3 py-2 text-sm font-medium transition-colors",
              active
                ? "bg-accent text-foreground"
                : "text-muted-foreground hover:bg-accent/60 hover:text-foreground",
            )}
          >
            {link.label}
          </a>
        {/each}

        <div class="mt-2 flex flex-col gap-2 border-t border-border/60 pt-3">
          {#if isAuthenticated}
            <div class="flex items-center gap-2.5 px-1 py-1.5">
              <UserAvatar size="default" />
              <div class="grid min-w-0 flex-1 leading-tight">
                <span class="truncate text-sm font-medium">{account?.name}</span>
                <span class="truncate text-xs text-muted-foreground">{account?.username}</span>
              </div>
            </div>
            <Button variant="outline" href="/ai/chats">Dashboard</Button>
            <Button variant="ghost" onclick={() => msalStore.switchAccount()}>Switch account</Button>
            <Button
              variant="ghost"
              class="text-destructive hover:text-destructive"
              onclick={() => msalStore.logoutRedirect()}
            >
              Sign out
            </Button>
          {:else}
            <Button disabled={signingIn} onclick={signIn}>
              {#if signingIn}
                <Icon icon="fa6-solid:spinner" class="size-4 animate-spin" />
              {/if}
              Sign in
            </Button>
          {/if}
        </div>
      </nav>
    </div>
  {/if}
</header>
