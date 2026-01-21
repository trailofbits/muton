use std::sync::Arc;

use mewt::LanguageRegistry;
use mewt::run_main;

mod languages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create language registry and register supported languages
    let mut registry = LanguageRegistry::new();
    registry.register(languages::func::engine::FuncLanguageEngine::new());
    registry.register(languages::tact::engine::TactLanguageEngine::new());

    // Run the shared main function
    run_main(Arc::new(registry)).await?;
    Ok(())
}
