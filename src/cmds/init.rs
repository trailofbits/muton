use std::fs;
use std::io::Write;
use std::path::PathBuf;

use log::{info, warn};

use crate::types::MutonResult;
use crate::types::config::default_global_config;

pub async fn execute_init() -> MutonResult<()> {
    info!("Initializing Muton workspace...");

    let cfg_path = PathBuf::from("muton.toml");
    if cfg_path.exists() {
        warn!("muton.toml already exists; leaving it unchanged");
    } else {
        let defaults = default_global_config();
        let toml = toml::to_string_pretty(&defaults)
            .map_err(|e| crate::types::MutonError::Custom(e.to_string()))?;
        let mut f = fs::File::create(&cfg_path)?;
        f.write_all(toml.as_bytes())?;
        info!("Created {}", cfg_path.display());
    }

    Ok(())
}
