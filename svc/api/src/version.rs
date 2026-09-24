//! Build information baked in by `build.rs`.
//!
//! Values fall back to `unknown` when the build had neither a git checkout nor
//! the matching variable in its environment.

/// Version string for logs, such as `0.1.0 (1132a66e1 2026-09-24)`.
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("LERPZ_COMMIT_SHORT_HASH"),
    " ",
    env!("LERPZ_COMMIT_DATE"),
    ")"
);

/// Full commit the binary was built from, for matching a log line to source.
pub const COMMIT_HASH: &str = env!("LERPZ_COMMIT_HASH");
