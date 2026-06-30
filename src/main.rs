use std::sync::Arc;

use mewt::LanguageRegistry;
use mewt::run_main;
use muton::languages;

const MUTON_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create language registry and register supported languages
    let mut registry = LanguageRegistry::new();
    registry.register_resolver(languages::func::resolver::FuncLanguageResolver::new());
    registry.register_resolver(languages::tact::resolver::TactLanguageResolver::new());
    registry.register_resolver(languages::tolk::resolver::TolkLanguageResolver::new());

    // Run the shared main function
    run_main(
        Arc::new(registry),
        "muton",
        "Mutation Testing Framework for TON Blockchain Languages",
        Some(MUTON_VERSION),
    )
    .await?;
    Ok(())
}
