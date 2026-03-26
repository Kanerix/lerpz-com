use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::agent::Agent;
use crate::config::CONFIG;
use crate::error::Result;
use crate::oai::PortkeyConfig;

mod agent;
mod config;
mod error;
mod oai;
mod tools;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!(
                "{}=debug,lerpz=debug,none",
                env!("CARGO_CRATE_NAME")
            ))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(debug_assertions)]
    {
        use std::path::PathBuf;

        let env_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".env"]);
        if let Err(err) = dotenvy::from_path(&env_path) {
            tracing::warn!("failed loading .env file: {}", err);
        }
    }

    let config = PortkeyConfig {
        api_base: CONFIG.PORTKEY_BASE_URL.clone(),
        api_key: CONFIG.PORTKEY_API_KEY.clone(),
        api_provider: CONFIG.PORTKEY_PROVIDER.clone(),
    };

    let agent = Agent::new(
        config,
        &CONFIG.DEFAULT_MODEL,
        "You are a helpful assistant. Use the search_knowledge_base tool \
         to find relevant information before answering questions. \
         Always cite your sources.",
    );

    let response = agent.run("Who am i?").await?;
    println!("Agent response:\n{response}");

    Ok(())
}
