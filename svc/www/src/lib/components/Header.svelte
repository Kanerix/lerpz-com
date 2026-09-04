<script lang="ts">
import Icon from "@iconify/svelte";
import { Button } from "@lerpz/ui/components/button";
import { cn } from "@lerpz/ui/lib/utils";
import { asset, resolve } from "$app/paths";
import { page } from "$app/state";
import ThemeButton from "$lib/components/ThemeButton.svelte";
import { site } from "$lib/config.js";

const navLinks = [
    { label: "Platform", href: resolve("/#platform") },
    { label: "About", href: resolve("/about") },
    { label: "Contact", href: resolve("/contact") },
    { label: "Portfolio", href: site.portfolioUrl, external: true },
];

const pathname = $derived(page.url.pathname);

function isActive(href: string): boolean {
    if (href.includes("#")) return false;
    return pathname === href || pathname.startsWith(`${href}/`);
}

let mobileOpen = $state(false);

$effect(() => {
    void pathname;
    mobileOpen = false;
});
</script>

<header class="sticky top-0 z-50 border-b border-border/60 bg-background/80 backdrop-blur-md">
  <div class="mx-auto flex h-16 max-w-5xl items-center justify-between gap-4 px-4">
    <a
      href={resolve("/")}
      class="flex shrink-0 items-center gap-2.5 font-semibold tracking-tight text-foreground transition-opacity hover:opacity-80"
    >
      <img src={asset("/lerpz.svg")} alt="Lerpz Logo" class="h-7 w-7" />
    </a>

    <nav class="hidden items-center gap-1 md:flex">
      {#each navLinks as link}
        {@const active = isActive(link.href)}
        <a
          href={link.href}
          target={link.external ? "_blank" : undefined}
          rel={link.external ? "noopener noreferrer" : undefined}
          aria-current={active ? "page" : undefined}
          class={cn(
            "inline-flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium transition-colors",
            active
              ? "bg-accent text-foreground"
              : "text-muted-foreground hover:bg-accent/60 hover:text-foreground",
          )}
        >
          {link.label}
          {#if link.external}
            <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-2.5 opacity-60" />
          {/if}
        </a>
      {/each}
    </nav>

    <div class="flex items-center gap-1.5">
      <ThemeButton />

      <Button class="hidden sm:inline-flex" href={site.appUrl}>
        Open the app
        <Icon icon="fa6-solid:arrow-right" class="size-4" />
      </Button>

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
      <nav class="mx-auto flex max-w-5xl flex-col gap-1 px-4 py-3">
        {#each navLinks as link}
          {@const active = isActive(link.href)}
          <a
            href={link.href}
            target={link.external ? "_blank" : undefined}
            rel={link.external ? "noopener noreferrer" : undefined}
            aria-current={active ? "page" : undefined}
            class={cn(
              "inline-flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium transition-colors",
              active
                ? "bg-accent text-foreground"
                : "text-muted-foreground hover:bg-accent/60 hover:text-foreground",
            )}
          >
            {link.label}
            {#if link.external}
              <Icon icon="fa6-solid:arrow-up-right-from-square" class="size-2.5 opacity-60" />
            {/if}
          </a>
        {/each}

        <div class="mt-2 flex flex-col gap-2 border-t border-border/60 pt-3">
          <Button href={site.appUrl}>Open the app</Button>
        </div>
      </nav>
    </div>
  {/if}
</header>
