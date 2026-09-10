---
name: frontend
description: Build or change user interface in svc/app, svc/www or packages/ui, following the Lerpz brand guide. Use when adding a page, component or layout, restyling something, or reviewing whether a design matches the house look.
---

# Building frontend

Read [`docs/BRAND.md`](../../../docs/BRAND.md) before writing any markup. It
defines the typeface, the colour tokens, the type scale and the layout traits.
`AGENTS.md` covers the Svelte and TypeScript conventions. This file covers how
to apply them when building an interface.

## Before you start

1. Find the closest existing screen or component and match it. `svc/www` has
   marketing sections in `src/lib/components`, `svc/app` has product surfaces in
   `src/lib/components` and `src/routes/(dashboard)`.
2. Check whether `@lerpz/ui` already has the component. There are 18 of them,
   including button, card, dialog, dropdown-menu, input, popover, select,
   sidebar, skeleton, tooltip and textarea. Do not rebuild one.
3. Decide where it belongs. Shared between both apps means `packages/ui`, and
   the `typescript-package` skill covers that wiring. Used by one app means that
   app's `src/lib/components`.

## Rules that are easy to get wrong

- **Semantic tokens only.** `bg-card`, `text-muted-foreground`,
  `border-border`. Never `gray-500`, never a hex value, never `dark:` with a
  hand-picked colour. The tokens already carry both themes.
- **The accent is scarce.** `text-primary` or `bg-primary` marks one thing per
  view. If two elements compete for it, one of them is wrong.
- **Headings are already styled.** A bare `<h2>` is correct. Add utilities only
  for a deliberate responsive step, and keep `tracking-tight` when scaling up.
- **Do not sort Tailwind classes.** The rule is off on purpose. Leave the order
  as written and let Biome format the rest.
- **Compose with `cn()`** from `@lerpz/ui/lib/utils`, and put variants in a
  sibling `*-variants.ts` using CVA. Accept a `class` prop, destructured as
  `class: className`, and merge it last so callers can override.
- **Import transitions from `$lib/utils/transitions.js`**, not from
  `svelte/transition`, so reduced motion is respected.
- **Never remove a focus ring.** Interactive elements keep a visible
  `focus-visible` state.

## Structure

- Page container: `mx-auto w-full max-w-5xl px-4`.
- Section: `py-16 md:py-24`, and `scroll-mt-24` if it is a scroll target.
- Space with flex or grid and `gap`, not margins.
- Separate content with a hairline, often `border-border/60`, rather than
  wrapping everything in another bordered card.
- Icons are `@iconify/svelte` with a string name from the `fa6-*` sets. Inside a
  button the size is automatic, elsewhere use `size-4` or `size-4.5`.

## A typical section

```svelte
<section class="flex flex-col gap-8 py-16 md:py-24">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">Title</h2>
    <p class="leading-relaxed text-muted-foreground">One supporting sentence.</p>
  </div>

  <div class="grid gap-x-12 gap-y-2 sm:grid-cols-2">
    <!-- items separated by border-t border-border/60 -->
  </div>
</section>
```

## Copy

Interface text follows the writing rules in `AGENTS.md`. Short, plain, no
filler, no em dashes. Write the label a user would say out loud. Button labels
are verbs, such as "Open the app" rather than "Submit".

## Before reporting back

- Check both themes. `mode-watcher` drives the `.dark` class, so nothing should
  need a hand-written dark variant.
- Check the layout at a narrow width. The responsive steps are `sm:`, `md:` and
  occasionally `lg:`.
- Run `just check-ts`, which runs `svelte-check` and Biome.
