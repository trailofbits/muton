use log::info;

use crate::store::MutonStore;
use crate::types::MutonResult;

pub async fn execute(store: MutonStore, mutant_id: i64) -> MutonResult<()> {
    info!("Getting mutant with id: {mutant_id}");
    let mutant = store.get_mutant(mutant_id).await?;
    let target = store.get_target(mutant.target_id).await?;
    let mutated_target = target.mutate(&mutant)?;
    info!("{mutated_target}");
    Ok(())
}
