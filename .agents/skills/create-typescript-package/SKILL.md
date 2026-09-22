---
name: typescript-package
description: Add a new shared TypeScript or Svelte package to the workspace. Use when code or configuration needs to be shared between frontends.
---

# Adding a TypeScript package

Shared packages live in `packages/`. The root `package.json` already lists
`packages/*` as a workspace, so it does not need editing. Use `packages/ui` as
the reference for a component package and `packages/biome-config` for a
config-only one.

Add a package only when the code is genuinely shared. Code used by one service
belongs in that service.

## 1. Name it

`@lerpz/<thing>` in a kebab-case directory, `packages/<thing>`.

## 2. Manifest

Packages ship source. There is no build step and no `dist`. A helper or config
package exposes subpaths only. A component package also exposes a root export
that re-exports every component, because `AGENTS.md` requires consumers to take
components from the root and keep subpaths for styles, hooks and helpers.

```json
{
    "name": "@lerpz/foo",
    "version": "0.0.0",
    "private": true,
    "type": "module",
    "scripts": {
        "check": "tsc --noEmit",
        "lint": "biome check",
        "format": "biome format --write"
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

The `check`, `lint` and `format` script names matter, the root
`bun --filter '*'` scripts call them by name. A package that omits one is
silently skipped by `just check` or `just fmt`.

A Svelte package uses `"check": "svelte-check --tsconfig ./tsconfig.json"`
instead, takes `svelte` as a `peerDependency` on `^5.0.0`, and adds a root
export alongside the component subpath:

```json
"exports": {
    ".": "./src/index.ts",
    "./components/*": "./src/components/*/index.ts"
}
```

`src/index.ts` re-exports each component directory barrel, which is what makes
`import { Button } from "@lerpz/ui"` work.

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

`packages/ui/tsconfig.json` extends the shared base. Do not hand-copy the
options into a package instead, which silently drops `noUncheckedIndexedAccess`
and the declaration settings.

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
`bun install`. Import helpers through the subpath, such as
`import { thing } from "@lerpz/foo/lib/thing"`, and components from the root,
such as `import { Button } from "@lerpz/ui"`.

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
