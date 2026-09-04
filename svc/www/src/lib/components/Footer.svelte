<script lang="ts">
import Icon from "@iconify/svelte";
import { asset, resolve } from "$app/paths";
import { site } from "$lib/config.js";

const footerLinks = {
    Product: [
        { label: "Platform", href: resolve("/#platform") },
        { label: "Open the app", href: site.appUrl },
    ],
    Company: [
        { label: "About", href: resolve("/about") },
        { label: "Contact", href: resolve("/contact") },
    ],
    Resources: [
        { label: "Portfolio", href: site.portfolioUrl },
        { label: "GitHub", href: site.githubUrl },
        { label: "Email me", href: `mailto:${site.contactEmail}` },
    ],
};
</script>

<footer class="border-t border-border/60 bg-card/40">
  <div class="mx-auto max-w-5xl px-4 py-12">
    <div class="grid grid-cols-2 gap-10 md:grid-cols-4">
      <div class="col-span-2 flex flex-col gap-4 md:col-span-1">
        <a href={resolve("/")} class="flex items-center gap-2 font-semibold text-foreground transition-opacity hover:opacity-75">
          <img src={asset("/lerpz.svg")} alt="Lerpz Logo" class="h-5 w-5" />
          <span class="text-sm tracking-tight">{site.name}</span>
        </a>
        <p class="text-sm leading-relaxed text-muted-foreground">
          The platform for building AI-powered experiences, maintained in one
          place so every team works from the same tooling.
        </p>
        <a
          href={site.githubUrl}
          target="_blank"
          rel="noopener noreferrer"
          aria-label="GitHub"
          class="w-fit text-muted-foreground transition-colors hover:text-foreground"
        >
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
      <p>© 2026 {site.name}. All rights reserved.</p>
      <p class="flex items-center gap-1">
        Made with <Icon icon="fa6-solid:heart" class="size-3.5 text-destructive" aria-label="love" /> by one developer
      </p>
    </div>
  </div>
</footer>
