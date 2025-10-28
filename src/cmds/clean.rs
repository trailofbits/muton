use log::{info, warn};

use crate::store::MutonStore;
use crate::types::{Hash, MutonResult};

pub async fn execute_clean(store: MutonStore) -> MutonResult<()> {
    info!("Cleaning database of stale targets...");

    // Get all targets from the database
    let targets = store.get_all_targets().await?;
    info!("Found {} targets in database", targets.len());

    let mut removed_count = 0;

    for target in targets {
        // Check if the file exists
        let path = &target.path;
        if !path.exists() {
            info!("Target file no longer exists: {}", target.display());
            store.remove_target(target.id).await?;
            removed_count += 1;
            continue;
        }

        // Read the current file content
        match std::fs::read_to_string(path) {
            Ok(current_content) => {
                let current_hash = Hash::digest(current_content);

                // Compare with stored hash
                if current_hash.to_hex() != target.file_hash.to_hex() {
                    info!("Target file hash has changed: {}", target.display());
                    store.remove_target(target.id).await?;
                    removed_count += 1;
                }
            }
            Err(e) => {
                warn!("Could not read file {}: {}", path.display(), e);
                // Remove targets we can no longer read
                store.remove_target(target.id).await?;
                removed_count += 1;
            }
        }
    }

    info!("Cleanup complete. Removed {removed_count} stale targets.");

    Ok(())
}
