# @lerpz/biome-config

Shared [Biome](https://biomejs.dev) configuration for the Lerpz monorepo.

## Exports

- `@lerpz/biome-config/svelte` — settings for Svelte projects that use
  Tailwind CSS (CSS Tailwind directives, the Svelte linter domain, Svelte
  file overrides, and Tailwind class sorting).

## Usage

In a package's `biome.json`:

```json
{
    "$schema": "https://biomejs.dev/schemas/2.5.0/schema.json",
    "root": false,
    "extends": ["@lerpz/biome-config/svelte"],
    "files": {
        "includes": ["**"]
    }
}
```

The preset is self-contained: it carries the shared formatter, JavaScript,
assist, and lint settings on top of the Svelte + Tailwind specifics, so a
consuming project only needs to declare its own `files.includes`.

> Note: Biome's `"extends": "//"` root-inheritance microsyntax cannot be
> combined with a package specifier in the same array, which is why this
> preset is self-contained rather than a thin delta over the monorepo root.
