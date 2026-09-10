# Brand

This document describes how the Lerpz frontends look and why. It covers the
typeface, the colour tokens, the type scale and the layout traits shared by
`svc/www` and `svc/app`.

Everything here is implemented in
[`packages/ui/src/styles/globals.css`](../packages/ui/src/styles/globals.css).
That file is the single source of truth. Both apps import it as the first line
of their `src/app.css` and add nothing but app-specific rules. If a value in
this document and a value in that file disagree, the file is right and this
document needs updating.

There is no `tailwind.config` file. Tailwind v4 is configured in CSS.

## Typeface

The platform uses one typeface for everything: **Recursive**, loaded from Google
Fonts. Sans, mono and serif all resolve to it, so there is no second family to
pair with and no font stack to maintain.

Recursive is a variable font with four axes beyond weight, and the design uses
them instead of loading separate faces:

| Axis | Custom property | Default | Used for |
|---|---|---|---|
| `MONO` | `--rec-mono` | `0` | Monospace forms. Set to `1` on `code` and `pre` |
| `CASL` | `--rec-casl` | `0` | Casual, brush-like letterforms |
| `slnt` | `--rec-slnt` | `0` | Slant, down to `-15` |
| `CRSV` | `--rec-crsv` | `0` | True cursive italic forms |

A `*` rule re-resolves these inherited custom properties into
`font-variation-settings` on every element, which is what makes them compose.
`wght` is deliberately left out so ordinary `font-weight` still works.

Five utilities set the axes, and they can be combined, for example
`recursive-mono recursive-casual`:

`recursive-sans`, `recursive-mono`, `recursive-linear`, `recursive-casual`,
`recursive-slant`.

Two behaviours are automatic and should not be reimplemented:

- `em` and `i` get Recursive's true italic, meaning slant `-14` plus cursive
  letterforms, with `font-style: normal` so the browser does not also synthesise
  an oblique.
- `code` and `pre` switch to the mono axis.

Note that the heading rule sets `--rec-casl: 0`, so headings currently render
linear rather than casual, despite the comment above it. Treat headings as
linear until that is deliberately changed.

## Type scale

Headings are styled globally in the base layer, so a plain `<h2>` is already
correct. Reach for utility classes only when a specific block needs to deviate.

| Element | Size | Line height | Letter spacing | Weight |
|---|---|---|---|---|
| `h1` | 2.25rem | 1.15 | -0.025em | 700 |
| `h2` | 1.875rem | 1.25 | -0.02em | 600 |
| `h3` | 1.5rem | 1.3 | -0.015em | 600 |
| `h4` | 1.25rem | 1.4 | -0.01em | 500 |
| `h5` | 1.125rem | 1.5 | 0 | 500 |
| `h6` | 1rem | 1.5 | 0 | 500 |

Body text is 1.65 line height. Long-form paragraphs add `leading-relaxed` and a
measure limit such as `max-w-xl` or `max-w-2xl`.

The pattern is negative tracking that decreases as headings get smaller, and
weight that steps down from bold to medium. When a heading is scaled up, keep
`tracking-tight`.

Marketing pages override the scale responsively rather than using the defaults:

- Page title: `text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl`
- Section title: `text-2xl font-semibold tracking-tight md:text-3xl`
- Card or list item title: `text-sm font-semibold`

An eyebrow above a title is `text-sm font-medium text-muted-foreground uppercase
tracking-widest`, or `text-xs` in a denser panel. Service and code identifiers
are set in `font-mono`.

## Colour

The palette is a warm neutral base with a single blue accent. Light mode is a
warm off-white rather than pure white, and dark mode is a warm dark grey rather
than black. Colours are written in `oklch`, with one exception noted below.

Never use a raw palette colour such as `gray-500` or a hex value in a component.
Use the semantic token, so both themes stay correct.

| Token | Purpose |
|---|---|
| `background`, `foreground` | Page base and default text |
| `card`, `card-foreground` | Raised surfaces |
| `popover`, `popover-foreground` | Floating surfaces such as menus and dialogs |
| `primary`, `primary-foreground` | The blue accent, and text on it |
| `secondary`, `secondary-foreground` | Quieter filled elements |
| `muted`, `muted-foreground` | Subdued backgrounds and secondary text |
| `accent`, `accent-foreground` | Hover and active states |
| `destructive` | Errors and destructive actions |
| `border`, `input`, `ring` | Hairlines, field chrome, focus rings |
| `chart-1` to `chart-5` | Data visualisation series |
| `sidebar-*` | Sidebar surface, which mostly aliases the tokens above |

`--primary` is `#6482fa`, the one literal hex in the file, and it is identical in
light and dark. It is the only colour that carries brand weight, so use it
sparingly: a primary action, an active state, an icon accent, or a single
highlighted phrase in a heading.

`--ring` is a blue close to the primary and is shared by both themes.
`--destructive` differs between themes: near-black in light, red in dark.

The browser theme colour is declared in each `app.html` as `#f8f6ef` for light
and `#262624` for dark. If the background token changes, change those too.

## Shape and depth

`--radius` is `0.625rem`, and the scale is derived from it, from `radius-sm` at
0.6 times up to `radius-4xl` at 2.6 times. Buttons and cards use `rounded-lg`,
which is the base radius. Larger panels use `rounded-xl`.

Shadows are deliberately shallow. Every step from `shadow-2xs` to `shadow-2xl`
is a 1px to 10px offset at 5 to 25 percent opacity. Cards use `shadow-sm` and
separation usually comes from a border rather than a shadow.

Borders do a lot of the work. A hairline at `border-border`, often softened to
`border-border/60`, is the normal way to divide content. Interface density comes
from spacing and rules, not boxes inside boxes.

## Layout

- The page container is `mx-auto w-full max-w-5xl px-4`, with `py-12 md:py-16`.
- A content section is `py-16 md:py-24`.
- Spacing between elements uses flex or grid with `gap`, not margins.
- `scrollbar-gutter: stable` is set on `html` in both apps so the centred
  content does not shift between a scrolling and a non-scrolling route.
- A section that is a scroll target gets `scroll-mt-24`.

Focus states are never removed. The base layer sets `outline-ring/50` globally,
and interactive components use a 3px `focus-visible` ring.

## Motion

Motion is small and quick. Animation utilities come from `tw-animate-css`, and
`mode-watcher` handles the theme switch.

Reduced motion is respected in two places, and both already exist:

- Each app's `app.css` reduces CSS animation and transition durations to
  0.01ms under `prefers-reduced-motion`, while keeping them technically running
  so state changes still happen.
- `svc/app/src/lib/utils/transitions.ts` wraps Svelte's `slide`, `fly` and
  `fade`. `slide` becomes instant, `fly` degrades to a plain fade because
  movement is the essential part, and `fade` runs unchanged because a fade is
  not motion. Import transitions from there rather than from `svelte/transition`.

## Icons

Icons come from `@iconify/svelte`, referenced by string name, mostly from the
Font Awesome 6 sets such as `fa6-solid:arrow-right` and `fa6-regular:comment`.
The default icon size inside a button is `size-4`, which the button variants
apply automatically. An accent icon takes `text-primary`.

## Traits

If a design decision is not covered above, these are the traits to preserve.

- **Warm, not clinical.** The neutrals are warm and the base is off-white. Avoid
  pure white, pure black and cool greys.
- **One accent.** Blue marks the single most important thing on the screen.
  A screen with accents everywhere has none.
- **Lines over boxes.** Separate content with a hairline and space. Do not nest
  bordered cards inside bordered cards.
- **Flat and quiet.** Shallow shadows, no gradients on surfaces, no heavy fills.
  The one exception is the soft primary glow behind the marketing layout, which
  appears in dark mode only.
- **Dense but breathing.** Tight tracking on headings, generous line height in
  body text, and consistent gaps.
- **Typography does the work.** With one typeface and one accent, hierarchy
  comes from size, weight and colour value rather than decoration.
- **Accessible by default.** Visible focus rings, honoured reduced-motion
  preferences, and both themes checked.

Copy follows the writing rules in [`AGENTS.md`](../AGENTS.md): short, plain and
direct, with no filler and no em dashes.
