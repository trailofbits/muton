use log::info;

use crate::store::MutonStore;
use crate::types::{Mutant, MutonResult, Outcome, Status, Target};

// Print outcome details and verbose information if requested
fn print_outcome(mutant: &Mutant, target: &Target, outcome: &Outcome, verbose: bool) {
    info!(
        "  {:<9} | {}",
        &outcome.status.display(),
        mutant.display(target)
    );

    // Print output & timing info if verbose
    if verbose {
        info!(
            "  Executed at: {}, Duration: {}ms",
            outcome.time, outcome.duration_ms
        );
        if !outcome.output.is_empty() {
            info!(
                "{}",
                outcome
                    .output
                    .trim()
                    .lines()
                    .map(|line| format!("  {line}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }
}

pub async fn execute(
    store: MutonStore,
    target_path: Option<String>,
    verbose: bool,
    mutant_id: Option<i64>,
    all: bool,
) -> MutonResult<()> {
    // Get targets filtered by path
    let filtered_targets = Target::filter_by_path(&store, target_path.clone()).await?;
    if filtered_targets.is_empty() {
        info!("No targets found");
        return Ok(());
    }

    // If mutant_id is provided, fetch and show only that specific mutant's outcome
    if let Some(id) = mutant_id {
        // Get the mutant
        match store.get_mutant(id).await {
            Ok(mutant) => {
                // Get the target for this mutant
                let target = store.get_target(mutant.target_id).await?;
                info!("Target: {}", target.display());

                // Get the outcome for this mutant
                if let Some(outcome) = store.get_outcome(mutant.id).await? {
                    print_outcome(&mutant, &target, &outcome, verbose);
                } else {
                    info!("  No outcome found for mutant with ID: {id}");
                }
            }
            Err(_) => {
                info!("Mutant with ID {id} not found");
            }
        }
        return Ok(());
    }

    // Group outcomes by target
    for target in filtered_targets {
        info!("Target: {}", target.display());

        // Get all mutants for this target
        let mut mutants = store.get_mutants(target.id).await?;
        // Sort by byte_offset so earlier mutations in the file come first
        mutants.sort_by_key(|m| m.byte_offset);
        if mutants.is_empty() {
            info!("  No mutants found for this target");
            continue;
        }

        // Retrieve outcomes for each mutant
        let mut has_outcomes = false;
        let mut eligible_total: u32 = 0;
        let mut caught_count: u32 = 0;
        for mutant in mutants {
            // Get the outcome for this mutant
            if let Some(outcome) = store.get_outcome(mutant.id).await? {
                // Update counters first (exclude Skipped and BuildFail from stats)
                let status = outcome.status.clone();
                if status != Status::Skipped && status != Status::BuildFail {
                    eligible_total += 1;
                    if status == Status::TestFail {
                        caught_count += 1;
                    }
                }

                // Show all outcomes if verbose, all flag is set, or only uncaught outcomes otherwise
                if verbose || all || status == Status::Uncaught {
                    has_outcomes = true;
                    print_outcome(&mutant, &target, &outcome, verbose);
                }
            }
        }

        if !has_outcomes {
            info!("  No outcomes found for this target");
        }

        // Print summary percentage of caught mutants for this target
        let percent = if eligible_total > 0 {
            (caught_count as f64 / eligible_total as f64) * 100.0
        } else {
            0.0
        };
        info!("Caught: {percent:.1}% ({caught_count} / {eligible_total})");
        info!(""); // Empty line between targets
    }

    Ok(())
}
