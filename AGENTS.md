# lerpz.com - Agent Specification

This is a general-purpose mono repository for the Lerpz organisation.

## Personas

Unless told otherwise, work as the developer persona described in
`.agents/personas/developer.md`. Read it alongside this file. It covers how to
work, this file covers the conventions.

The other personas in `.agents/personas/` are used when the task calls for them:
`researcher.md` for read-only questions about the codebase, and `reviewer.md`
for reviewing a diff or branch.

## General rules

Keep things simple. Solve the problem that was asked for, in the smallest way
that fits the code around it.

- Do not build for requirements that do not exist yet. No extra abstraction
  layers, configuration options or generic helpers added on the assumption that
  something will need them later.
- Prefer the boring solution. A plain function beats a trait, a macro or a
  generic parameter unless there is a concrete reason for the latter.
- Change as little as possible. Do not rename, reformat or restructure code that
  is unrelated to the task.
- Reuse what is already in the repository before reaching for a new dependency,
  pattern or file.
- If something cannot be done simply, say so and explain the trade-off rather
  than hiding the complexity in clever code.
- Leave working code behind, not scaffolding. Finish one thing properly instead
  of half-doing several.

## Writing

This applies to commit messages, documentation, comments, pull request
descriptions and anything else written for a human to read.

- Do not use em dashes or en dashes. Use a comma, a full stop, or rewrite the
  sentence.
- Write short, direct sentences in plain language, and say each thing once.
- Cut filler words such as "simply", "just", "seamlessly", "robust", "powerful"
  and "leverage".
- Drop the summary that repeats what was already said, and the closing sentence
  that adds nothing.
- Avoid the patterns that read as machine written, such as "it is not just X, it
  is Y", lists of three adjectives, and headings or bold text sprinkled through
  a few short paragraphs.
- Use British spelling, as the rest of the repository does.
- Do not overstate. If something was not tested or verified, write that instead
  of claiming it works.

## Git conventions

Commit messages follow the [Conventional Commits](https://www.conventionalcommits.org)
specification and should be written in the imperative mood, as recommended by Git.

## Comments

Prefer self-explanatory code over comments. Before writing a comment, try to make
it unnecessary. Use descriptive names for variables, functions, parameters and
types, extract a named function or constant, or restructure the code so the
intent is obvious.

Write a comment only when it explains something the code cannot, such as:

- Why a non-obvious approach was chosen, or why the obvious one does not work.
- Constraints, trade-offs, invariants and assumptions that are not visible locally.
- References to external context, such as a specification, ticket or upstream bug.
- Warnings about surprising behaviour or subtle edge cases.

Do not write comments that:

- Restate what the code already says.
- Narrate a change or its history. That belongs in the commit message.
- Mark a section of a file, for example:

    ```
    // ---------------------- Stuff here ----------------------
    ```

    Split the code into smaller functions, modules or files instead.

Documentation comments on public APIs are encouraged and are not covered by the
rules above, but they should explain purpose, behaviour and caveats rather than
repeat the signature. Follow the language's documentation conventions and keep
existing comments up to date when the surrounding code changes.

## Language rules

`just fmt` runs `cargo fmt` and Biome, and `just check` runs Clippy,
`svelte-check` and Biome. Leave formatting to those tools and do not hand-format
code to look different from what they produce.

### Rust

Edition 2024 on the nightly toolchain. Every dependency is declared in
`[workspace.dependencies]` and pulled in with `{ workspace = true }`.

- `clippy::unwrap_used` is denied. Use `expect("lowercase reason")` or
  `unwrap_or_else(|err| panic!("...: {err}"))`. Do not silence the lint with an
  `#[allow]`.
- Library errors use `thiserror` in an `error.rs` next to the code they belong
  to, together with a `pub type Result<T> = std::result::Result<T, Error>` alias.
  `anyhow` is only an opaque sink for errors that get logged. It is never a
  crate's public error type.
- HTTP handlers return `lerpz_axum::problem::HandlerResult<T>`. Do not add a
  per-service error enum with its own `IntoResponse`. A bare `?` turns into a
  generic 500, so build a `Problem` explicitly when the caller deserves a real
  status. Attach the cause with `.with_error(err)`. The client only ever gets a
  `log_id`, never the underlying message.
- Modules use `mod.rs` instead of the `foo.rs` and `foo/` pairing. Crates declare
  `pub mod` in `lib.rs` and re-export their key types with `pub use`. Optional
  functionality lives behind an additive feature flag.
- One endpoint per file, exporting a single `pub async fn handler` with
  `#[utoipa::path(...)]` followed by `#[axum::debug_handler(state = AppState)]`.
  Register it in the parent `mod.rs` in `routes!(...)` and take the tag from a
  constant in `oapi.rs`. If the tag is new, add it to the service's
  `#[derive(OpenApi)]`. The frontend client is generated from the OpenAPI spec,
  so these annotations affect more than the docs page.
- Handlers take sub-state, such as `State(database): State<DatabasePool>`. A new
  `AppState` field therefore needs its own `impl FromRef<AppState>`. `Extension`
  is not used. Argument order is auth token, path and query, state, then body.
- Log with the `tracing` macros and structured fields, such as
  `tracing::info!(%user_id, "creating conversation")`. Messages are lowercase and
  in the present tense.
- Queries use the compile-time `query!`, `query_as!` and `query_scalar!` macros,
  and are written where they are used. The workspace compiles offline against the
  committed `.sqlx` cache, so changing SQL means running `just prepare` and
  committing the regenerated cache with it.
- Payload types are suffixed `Request` and `Response`, derive `ToSchema` and stay
  snake_case on the wire. Only reach for `rename_all` when an external contract
  needs it.
- Configuration goes through `lerpz_utils::generate_config!` behind a
  `LazyLock`, with secrets typed as `secrecy::SecretString`.
- Tests are inline `#[cfg(test)] mod tests` blocks. There is no `tests/`
  directory.

### TypeScript and Svelte

Bun workspaces, Svelte 5 and SvelteKit 2, checked with `svelte-check` and linted
and formatted by Biome.

- Runes only: `$props`, `$state`, `$derived` and `$effect`. `export let`,
  `<slot>`, `on:click` and `writable` or `readable` stores do not appear in this
  codebase and should not be introduced.
- Relative and `$lib` imports carry a `.js` extension, since
  `verbatimModuleSyntax` is on and the build is ESM. Type-only imports use
  `import type`. Prefer `type` aliases over `interface`.
- Components are PascalCase, one per file, inside kebab-case directories. Props
  are an inline object type on `$props()`, with JSDoc on any field that is not
  self-evident. Children are `children?: Snippet` rendered with
  `{@render children?.()}`, and callbacks are `onX` function props.
- Any module containing runes is named `*.svelte.ts`. State is either a class
  with `$state` fields exported as a singleton, or a `createX()` factory that
  returns getters. Context modules export a `Symbol` key with matching
  `setXContext` and `getXContext`, where the getter throws if the context is
  missing.
- Styling is Tailwind v4 configured in CSS, so there is no `tailwind.config`
  file. Use the semantic design tokens such as `bg-card` and
  `text-muted-foreground` rather than raw palette colours, compose classes with
  `cn()` from `@lerpz/ui/lib/utils`, and keep variants in a sibling
  `*-variants.ts` using CVA. Do not sort Tailwind classes, as the rule is off on
  purpose.
- Shared components come from `@lerpz/ui` through subpath imports such as
  `@lerpz/ui/components/button`. The package has no root export and no build
  step, and wraps `@ark-ui/svelte`.
- Read environment variables through the validated `publicEnv` export rather than
  `$env` directly.
- Authentication is client-side MSAL against Entra ID. There is no server-side
  SvelteKit code, so no `hooks.server.ts`, no `+page.server.ts` and no form
  actions. Do not put auth in a server hook. `svc/app` still renders on the
  server, so guard browser-only code with `browser` from `$app/environment`.
- `svc/app/src/lib/api` is generated from the API's OpenAPI spec by orval. It is
  committed and excluded from linting. Regenerate it with `just openapi` instead
  of editing it.

### SQL

Migrations live in the top-level `migrations/` directory and are created with
`just migration NAME`. Keywords are uppercase. Once a migration is merged it is
append-only, so fix a mistake with a new migration rather than editing one that
has already been applied.
