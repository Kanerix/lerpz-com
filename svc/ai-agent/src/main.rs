use std::io::BufRead;

use rig::completion::Prompt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::agent::build_agent;
use crate::config::CONFIG;
use crate::error::Result;

mod agent;
mod config;
mod error;
mod portkey;
mod tools;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        use std::path::PathBuf;
        let env_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".env"]);
        let _ = dotenvy::from_path(&env_path);
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!(
                "{}=debug,lerpz=debug,none",
                env!("CARGO_CRATE_NAME")
            ))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let agent = build_agent().await?;

    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        eprintln!("No input provided.");
        return Ok(());
    }

    tracing::debug!(%input, "Running agent");

    let response = agent.prompt(input).await?;
    println!("Agent response:\n{response}");

    Ok(())
}
