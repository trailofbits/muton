use std::collections::HashMap;
use std::str::FromStr;

use log::info;

use crate::mutations;
use crate::types::{Language, MutationSeverity};

pub async fn execute(language: Option<String>) -> Result<(), String> {
    match &language {
        Some(lang_str) => {
            // Parse the language string to our Language enum
            let lang = Language::from_str(lang_str)?;
            print_mutations_for_language(&lang);
        }
        None => {
            // For each language, print its mutations
            for lang in Language::all() {
                print_mutations_for_language(&lang);
            }
        }
    };

    Ok(())
}

fn print_mutations_for_language(lang: &Language) {
    let mutation_engine = mutations::get_mutations_for_language(lang);
    let mutations = mutation_engine.get_mutations();

    // Group mutations by slug
    let mut mutation_groups: HashMap<&str, (MutationSeverity, Vec<&str>)> = HashMap::new();

    for mutation in mutations {
        let entry = mutation_groups
            .entry(mutation.slug)
            .or_insert((mutation.severity.clone(), Vec::new()));
        entry.1.push(mutation.description);
    }

    // Sort slugs for consistent output
    let mut slugs: Vec<_> = mutation_groups.keys().copied().collect();
    slugs.sort();

    info!("Available mutations for {lang}:");
    for slug in slugs {
        let (severity, descriptions) = &mutation_groups[slug];
        if descriptions.len() == 1 {
            info!("  [{}] {} (Severity: {})", slug, descriptions[0], severity);
        } else {
            info!(
                "  [{}] {} (and {} other variants) (Severity: {})",
                slug,
                descriptions[0],
                descriptions.len() - 1,
                severity
            );
        }
    }
    info!("");
}
