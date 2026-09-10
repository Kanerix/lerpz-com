---
name: rust-crate
description: Add a new shared Rust crate under crates/ in this Cargo workspace, wired up with workspace dependencies, workspace lints, module layout and the thiserror error pattern. Use when a piece of Rust logic needs to be shared between services.
---

# Adding a Rust crate

Shared crates live in `crates/`. The workspace picks them up through
`members = ["crates/*"]`, so the root `[workspace]` list does not need editing.
Use `crates/lerpz-pwd` as the reference layout.

Add a crate only when the code is genuinely shared. Code used by one service
belongs in that service.

## 1. Name it

`lerpz-<thing>`, lowercase, single hyphens. Name the role, not the technology,
so the name survives a rewrite.

## 2. Manifest

`crates/lerpz-foo/Cargo.toml`:

```toml
[package]
name = "lerpz-foo"
edition = "2024"
version.workspace = true

[dependencies]
thiserror = { workspace = true }

[lints]
workspace = true
```

`lints.workspace = true` is required, it is what enforces `clippy`.

Every dependency is declared in the root `[workspace.dependencies]` and pulled
in with `{ workspace = true }`. A new third-party dependency goes into the root
manifest first, under the comment group it belongs to. Add the crate itself to
that list as well, so services can depend on it the same way:

```toml
lerpz-foo = { path = "crates/lerpz-foo" }
```

## 3. Errors

Put the error type in `crates/lerpz-foo/src/error.rs`, next to the code it
belongs to, with the matching `Result` alias:

```rust
/// Errors that can occur when doing the thing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to do the thing: {0}")]
    Thing(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;
```

`anyhow` is not a public error type. It is only an opaque sink for errors that
get logged.

## 4. Library root

`src/lib.rs` carries `//!` module docs, declares modules, then re-exports the
names callers need:

```rust
//! What this crate is for.

/// Errors that can occur when working with foo.
mod error;

pub use error::{Error, Result};
```

- Modules use `mod.rs`, not the `foo.rs` and `foo/` pairing.
- Optional functionality lives behind an additive feature flag, declared in
  `Cargo.toml` and gated with `#[cfg(feature = "...")]` in `lib.rs`. Document
  the flag in the module docs. Services opt in explicitly, for example
  `lerpz-axum = { workspace = true, features = ["azure", "oapi"] }`.
- Document public items, including struct fields and enum variants. Explain
  purpose and caveats rather than repeating the signature.
- There is no prelude module in this workspace, do not add one.

## 5. Tests

Inline `#[cfg(test)] mod tests` blocks in the file under test. There is no
`tests/` directory.

## 6. Check it

```sh
just fmt
just check-rust
```
