---
name: frontend
description: Build or restyle the frontend. Use for pages, components, layouts and brand guide checks.
---

# Building frontend

If the project has a brand or design guide, read it before writing any markup.
It decides the typeface, the colour tokens, the type scale and the layout
traits, and it outranks this file. The project's agent instructions cover the
language and framework conventions. This file covers how to apply them when
building an interface.

## Before you start

1. Find the closest existing screen or component and match it. Precedent in the
   codebase beats a fresh design every time.
2. Check whether the shared component library already has what you need. A
   button, card, dialog, input, select or tooltip almost always exists already.
   Do not rebuild one.
3. Decide where it belongs. Used by more than one app means the shared library,
   used by one means that app's own component directory.

## Rules that are easy to get wrong

- **Semantic tokens only.** Use the tokens the design system defines, such as
  card, muted foreground and border. Never a raw palette colour, never a hex
  value, never a hand-picked dark variant. The tokens already carry both themes.
- **The accent is scarce.** The primary colour marks one thing per view. If two
  elements compete for it, one of them is wrong.
- **Headings are already styled.** A bare heading element is correct. Add
  utilities only for a deliberate responsive step, and keep the tighter tracking
  when scaling up.
- **Leave class order alone** unless the project's linter sorts it for you. Let
  the formatter handle the rest and do not hand-format against it.
- **Compose classes with the project's merge helper**, and keep variants in a
  sibling variants module. Accept a `class` prop and merge it last so callers can
  override.
- **Use the project's transition and motion helpers** where they exist, so a
  reduced motion preference is respected.
- **Never remove a focus ring.** Interactive elements keep a visible
  focus-visible state.

## Structure

- Centre page content in a container with a fixed maximum width and horizontal
  padding.
- Give sections generous vertical rhythm, and a scroll margin when they are
  scroll targets.
- Space with flex or grid and `gap`, not margins.
- Separate content with a hairline border rather than wrapping everything in
  another bordered card.
- Take icons from the icon set the project already depends on, at a consistent
  size.

## A typical section

Shape, not exact values. Swap in the project's own tokens and scale.

```html
<section class="flex flex-col gap-8 py-16 md:py-24">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class="text-2xl font-semibold tracking-tight md:text-3xl">Title</h2>
    <p class="leading-relaxed text-muted-foreground">One supporting sentence.</p>
  </div>

  <div class="grid gap-x-12 gap-y-2 sm:grid-cols-2">
    <!-- items separated by a hairline border -->
  </div>
</section>
```

## Copy

Interface text follows the project's writing rules. Short, plain, no filler, no
dashes. Write the label a user would say out loud. Button labels are verbs, such
as "Open the app" rather than "Submit".

## Before reporting back

- Check both themes. Nothing should need a hand-written dark variant.
- Check the layout at a narrow width, at each responsive step the project uses.
- Run the project's type check and lint task for frontend code.
