<script lang="ts">
import { Button } from "@lerpz/ui/components/button";
import { msalStore } from "$lib/auth/msal.svelte.js";
import ThemeButton from "$lib/components/ThemeButton.svelte";

const navLinks = [
    { label: "Tools", href: "/ai" },
    { label: "Roadmap", href: "/roadmap" },
    { label: "Docs", href: "/docs" },
    { label: "Changelog", href: "/changelog" },
    { label: "Status", href: "/status" },
];

const isAuthenticated = $derived(msalStore.isAuthenticated);
</script>

<header class="sticky top-0 z-50 border-b border-border/60 bg-background/80 backdrop-blur-md">
  <div class="mx-auto flex max-w-[1024px] items-center justify-between px-4 py-3">
    <a href="/" class="flex items-center gap-2.5 font-bold text-foreground transition-opacity hover:opacity-80">
      <img src="/lerpz.svg" alt="Lerpz Logo" class="h-7 w-7" />
      <span class="text-lg tracking-tight">Lerpz AI</span>
    </a>

    <nav class="hidden items-center gap-6 md:flex">
      {#each navLinks as link}
        <a href={link.href} class="text-sm font-medium text-muted-foreground transition-colors hover:text-foreground">
          {link.label}
        </a>
      {/each}
    </nav>

    <div class="flex items-center gap-2">
      {#if isAuthenticated}
        <a href="/ai/chats" class="hidden text-sm font-medium text-muted-foreground transition-colors hover:text-foreground sm:block">
          Dashboard
        </a>
      {/if}
      <ThemeButton />
      <Button onclick={() => (isAuthenticated ? msalStore.switchAccount() : msalStore.loginRedirect())}>
        {isAuthenticated ? "Switch account" : "Sign in"}
      </Button>
    </div>
  </div>
</header>
