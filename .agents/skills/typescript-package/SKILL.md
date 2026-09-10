---
name: typescript-package
description: Add a new shared TypeScript or Svelte package under packages/ in this Bun workspace, with the right package.json exports map, tsconfig and Biome wiring. Use when code or configuration needs to be shared between the app and www frontends.
---

# Adding a TypeScript package

Shared packages live in `packages/`. The root `package.json` already lists
`packages/*` as a workspace, so it does not need editing. Use `packages/ui` as
the reference for a component package and `packages/biome-config` for a
config-only one.

Add a package only when something is shared between `svc/app` and `svc/www`.
Code used by one app belongs in that app's `src/lib`.

## 1. Name it

`@lerpz/<thing>` in a kebab-case directory, `packages/<thing>`.

## 2. Manifest

Packages ship source. There is no build step, no `dist` and no root export, so
consumers import through subpaths.

```json
{
    "name": "@lerpz/foo",
    "version": "0.0.0",
    "private": true,
    "type": "module",
    "scripts": {
        "check": "tsc --noEmit",
        "lint": "biome check"
    },
    "devDependencies": {
        "@lerpz/biome-config": "workspace:*",
        "@lerpz/typescript-config": "workspace:*",
        "typescript": "^5.9.2"
    },
    "exports": {
        "./lib/*": "./src/lib/*.ts"
    }
}
```

The `check` and `lint` script names matter, the root `bun --filter '*'` scripts
call them by name. A Svelte package uses
`"check": "svelte-check --tsconfig ./tsconfig.json"` instead, takes `svelte` as
a `peerDependency` on `^5.0.0`, and exports components as
`"./components/*": "./src/components/*/index.ts"`.

Do not add an export entry for a directory that does not exist.

## 3. Config files

`tsconfig.json`:

```json
{
    "extends": "@lerpz/typescript-config/base.json",
    "include": ["./**/*.ts"],
    "exclude": ["node_modules"]
}
```

`packages/ui/tsconfig.json` hand-copies its options instead of extending the
shared base, which drops `noUncheckedIndexedAccess`. Do not copy that.

`biome.json`:

```json
{
    "$schema": "https://biomejs.dev/schemas/2.5.0/schema.json",
    "root": false,
    "extends": ["@lerpz/biome-config/svelte"]
}
```

## 4. Wire it up

Add `"@lerpz/foo": "workspace:*"` to the consuming app's dependencies and run
`bun install`. Import through the subpath, such as
`import { thing } from "@lerpz/foo/lib/thing"`.

## 5. Source conventions

- Relative and `$lib` imports carry a `.js` extension. Directory barrel imports
  do not.
- Type-only imports use `import type`. Prefer `type` aliases over `interface`.
- Svelte components are PascalCase, one per file, in a kebab-case directory with
  an `index.ts` barrel:

  ```ts
  export { default as Button } from "./Button.svelte";
  export type { ButtonProps } from "./button-variants.js";
  ```

- Runes only. Props are an inline object type on `$props()`, children are
  `children?: Snippet` rendered with `{@render children?.()}`.
- Styling uses the semantic tokens from `@lerpz/ui/globals.css`, composed with
  `cn()`. Variants go in a sibling `*-variants.ts` using CVA.

## 6. Check it

```sh
just check-ts
```
