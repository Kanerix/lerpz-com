<script lang="ts">
import type { Snippet } from "svelte";
import { page } from "$app/state";
import Footer from "$lib/components/landing-page/Footer.svelte";
import Header from "$lib/components/landing-page/Header.svelte";

let { children }: { children: Snippet } = $props();

const legalLinks = [
    { label: "Privacy Policy", href: "/legal/privacy" },
    { label: "Terms of Service", href: "/legal/terms" },
    { label: "Cookie Policy", href: "/legal/cookies" },
];
</script>

<div class="relative flex min-h-screen flex-col">
  <Header />

  <main class="mx-auto w-full max-w-[1024px] flex-1 px-4 py-12 md:py-16">
    <div class="flex flex-col gap-10 md:flex-row md:gap-16">
      <aside class="md:w-48 md:shrink-0">
        <nav class="flex flex-col gap-4 md:sticky md:top-24">
          <h2 class="text-sm font-semibold uppercase tracking-wider text-muted-foreground">Legal</h2>
          <ul class="flex flex-col gap-4">
            {#each legalLinks as link}
              {@const active = page.url.pathname === link.href}
              <li>
                <a
                  href={link.href}
                  aria-current={active ? "page" : undefined}
                  class="text-lg transition-colors md:text-xl {active
                    ? 'font-medium text-foreground'
                    : 'text-muted-foreground hover:text-foreground'}"
                >
                  {link.label}
                </a>
              </li>
            {/each}
          </ul>
        </nav>
      </aside>

      <div class="min-w-0 flex-1">
        {@render children()}
      </div>
    </div>
  </main>

  <Footer />
</div>
