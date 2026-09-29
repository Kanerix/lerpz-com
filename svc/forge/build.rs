//! Bakes commit information into the binary.
//!
//! Every variable is emitted on every build, including when git is missing, so
//! that `env!` in the crate always resolves. A value already present in the
//! build environment wins over git, which is how the container build gets the
//! commit without a checkout.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Stands in for a value that neither the environment nor git could supply.
const UNKNOWN: &str = "unknown";

fn main() {
    // Cargo does not expose the workspace root to build scripts.
    // https://github.com/rust-lang/cargo/issues/3946
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("manifest should sit two levels below the workspace root");

    if let Some(git_dir) = resolve_git_dir(&workspace_root.join(".git")) {
        rerun_on_commit(&git_dir);
    }

    let log = git_log(workspace_root).unwrap_or_default();
    let mut fields = log.split_whitespace();

    emit("LERPZ_COMMIT_HASH", fields.next());
    emit("LERPZ_COMMIT_SHORT_HASH", fields.next());
    emit("LERPZ_COMMIT_DATE", fields.next());

    // `%(describe:tags)` is `<tag>` on a tagged commit, `<tag>-<distance>-g<hash>`
    // once the build has moved past one, and empty when no tag is reachable.
    let describe = fields.next();
    emit("LERPZ_LAST_TAG", describe.and_then(|d| d.split('-').next()));
    emit(
        "LERPZ_LAST_TAG_DISTANCE",
        describe.map(|d| d.split('-').nth(1).unwrap_or("0")),
    );
}

/// Emits one variable, preferring an explicit value from the build environment.
fn emit(key: &str, from_git: Option<&str>) {
    println!("cargo::rerun-if-env-changed={key}");

    let from_env = env::var(key).ok();
    let value = [from_env.as_deref(), from_git]
        .into_iter()
        .flatten()
        .find(|value| !value.is_empty())
        .unwrap_or(UNKNOWN);

    println!("cargo::rustc-env={key}={value}");
}

/// Resolves `.git`, which is a file pointing elsewhere inside a worktree.
fn resolve_git_dir(dot_git: &Path) -> Option<PathBuf> {
    if dot_git.is_dir() {
        return Some(dot_git.to_path_buf());
    }

    let contents = fs::read_to_string(dot_git).ok()?;
    let (_, git_dir) = contents.split_once("gitdir:")?;
    Some(PathBuf::from(git_dir.trim()))
}

/// Rebuilds the crate when the checked-out commit changes.
fn rerun_on_commit(git_dir: &Path) {
    let head = git_dir.join("HEAD");
    if !head.exists() {
        return;
    }
    println!("cargo::rerun-if-changed={}", head.display());

    // HEAD holds a bare commit when detached, in which case watching it is
    // enough. On a branch it holds `ref: <ref>`, and the file behind that ref
    // is what moves when a commit lands.
    let Ok(contents) = fs::read_to_string(&head) else {
        return;
    };
    let Some(git_ref) = contents.trim().strip_prefix("ref:") else {
        return;
    };

    // A worktree keeps its own HEAD but shares refs with the main checkout.
    let common_dir = fs::read_to_string(git_dir.join("commondir"))
        .map(|path| git_dir.join(path.trim()))
        .unwrap_or_else(|_| git_dir.to_path_buf());

    println!(
        "cargo::rerun-if-changed={}",
        common_dir.join(git_ref.trim()).display()
    );
}

/// Reads the commit fields from git, if it is installed and this is a checkout.
fn git_log(workspace_root: &Path) -> Option<String> {
    let output = Command::new("git")
        .current_dir(workspace_root)
        .args([
            "log",
            "-1",
            "--date=short",
            "--abbrev=9",
            "--format=%H %h %cd %(describe:tags)",
        ])
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}
