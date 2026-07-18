<script lang="ts">
import Icon from "@iconify/svelte";
import { publicEnv } from "$lib/env.js";

const footerLinks = {
    Product: [
        { label: "Changelog", href: "/changelog" },
        { label: "Roadmap", href: "/roadmap" },
        { label: "Status", href: "/status" },
    ],
    Resources: [
        { label: "Documentation", href: "/docs" },
        { label: "API Reference", href: `${publicEnv.PUBLIC_API_URL}/scalar` },
        { label: "Support", href: "/support" },
    ],
    Company: [
        { label: "About", href: "/about" },
        { label: "Contact", href: "/contact" },
    ],
    Legal: [
        { label: "Privacy Policy", href: "/legal/privacy" },
        { label: "Terms of Service", href: "/legal/terms" },
        { label: "Cookie Policy", href: "/legal/cookies" },
    ],
};
</script>

<footer class="border-t border-border/60 bg-card/40">
  <div class="mx-auto max-w-[1024px] px-4 py-12">
    <div class="grid grid-cols-2 gap-10 md:grid-cols-5">
      <div class="col-span-2 flex flex-col gap-4 md:col-span-1">
        <a href="/" class="flex items-center gap-2 font-semibold text-foreground transition-opacity hover:opacity-75">
          <img src="/lerpz.svg" alt="Lerpz Logo" class="h-5 w-5" />
          <span class="text-sm tracking-tight">Lerpz AI</span>
        </a>
        <p class="text-sm leading-relaxed text-muted-foreground">
          The AI portal for your organisation. Maintained centrally so everyone works from the same tooling.
        </p>
        <a href="https://github.com/lerpz" target="_blank" rel="noopener noreferrer" aria-label="GitHub" class="w-fit text-muted-foreground transition-colors hover:text-foreground">
          <Icon icon="fa6-brands:github" class="size-4" />
        </a>
      </div>

      {#each Object.entries(footerLinks) as [group, links]}
        <div class="flex flex-col gap-3">
          <h4 class="text-xs font-semibold uppercase tracking-wider text-foreground">{group}</h4>
          <ul class="flex flex-col gap-2">
            {#each links as link}
              <li>
                <a href={link.href} class="text-sm text-muted-foreground transition-colors hover:text-foreground">
                  {link.label}
                </a>
              </li>
            {/each}
          </ul>
        </div>
      {/each}
    </div>

    <div class="mt-10 flex flex-col items-center justify-between gap-3 border-t border-border/60 pt-6 text-xs text-muted-foreground sm:flex-row">
      <p>© 2026 Lerpz AI. All rights reserved.</p>
      <p class="flex items-center gap-1">
        Made with <Icon icon="fa6-solid:heart" class="size-3.5 text-destructive" aria-label="love" /> by the Lerpz team
      </p>
    </div>
  </div>
</footer>
