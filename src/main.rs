use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use clap::Parser;
use log::{debug, info, warn};

use crate::cli::{Args, Commands};
use crate::logging::init_logging;
use crate::store::MutonStore;
use crate::types::MutonResult;
use crate::types::config::{CliOverrides, config, init_with_overrides};

mod cli;
mod cmds;
mod logging;
mod mutations;
mod runner;
mod store;
mod types;

#[tokio::main]
async fn main() -> MutonResult<()> {
    let args = Args::parse();

    // Handle global arguments
    if let Some(cwd_arg) = args.cwd.as_ref() {
        let cwd = PathBuf::from(cwd_arg).canonicalize()?;
        let _ = env::set_current_dir(&cwd);
    }
    let cwd = env::current_dir()?;
    debug!("Current working directory: {}", cwd.display());

    // Build CLI overrides for config precedence
    let cli_overrides = CliOverrides {
        db: args.db.clone(),
        log_level: args.log_level.clone(),
        log_color: args.log_color.clone(),
        ignore_targets: args.ignore_targets.clone(),
        mutations_slugs: match &args.command {
            Commands::Run(run_args) => run_args.mutations.clone(),
            _ => None,
        },
        test_cmd: match &args.command {
            Commands::Run(run_args) => run_args.test_cmd.clone(),
            Commands::Test(test_args) => test_args.test_cmd.clone(),
            _ => None,
        },
        test_timeout: match &args.command {
            Commands::Run(run_args) => run_args.timeout,
            Commands::Test(test_args) => test_args.timeout,
            _ => None,
        },
    };

    // Initialize configuration (files, env, then CLI overrides)
    init_with_overrides(&cli_overrides);

    // Initialize logging after config so level/color are applied
    init_logging();
    info!("Good morning");

    // Initialize the database
    let db_path = &config().general.db;
    let db_file = PathBuf::from(db_path);

    if !db_file.exists() {
        debug!(
            "Database file doesn't exist. Creating it at: {}",
            db_file.display()
        );
        let file = std::fs::File::create(&db_file)?;
        drop(file);
    }

    let db_connection_string = format!("sqlite:{db_path}");
    debug!("Using database: {db_connection_string}");
    let store = MutonStore::new(db_connection_string).await?;

    // Setup running flag to handle signals from ctrl-c
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = Arc::clone(&running);

    ctrlc::set_handler(move || {
        warn!("Received Ctrl-C, cleaning up..");
        running_ctrlc.store(false, Ordering::SeqCst);
    })
    .expect("Error creating a Ctrl-C handler");

    // Dispatch to appropriate command
    match args.command {
        Commands::Run(run_args) => {
            cmds::execute_run(run_args, store, running).await?;
        }
        Commands::Mutate(mutate_args) => {
            cmds::execute_mutate(mutate_args, store).await?;
        }
        Commands::Clean => {
            cmds::execute_clean(store).await?;
        }
        Commands::Test(test_args) => {
            cmds::execute_test(test_args, store, running).await?;
        }
        Commands::Purge(purge_args) => {
            cmds::execute_purge(purge_args, store).await?;
        }
        Commands::Print {
            command: print_args,
        } => match print_args {
            cli::PrintArgs::Mutations(args) => {
                cmds::execute_print(cmds::print::PrintCommand::Mutations(args.language), None)
                    .await?
            }
            cli::PrintArgs::Results(args) => {
                cmds::execute_print(
                    cmds::print::PrintCommand::Results(
                        args.target,
                        args.verbose,
                        args.id,
                        args.all,
                    ),
                    Some(store),
                )
                .await?
            }
            cli::PrintArgs::Targets => {
                cmds::execute_print(cmds::print::PrintCommand::Targets, Some(store)).await?
            }
            cli::PrintArgs::Mutant(args) => {
                cmds::execute_print(cmds::print::PrintCommand::Mutant(args.id), Some(store)).await?
            }
            cli::PrintArgs::Mutants(args) => {
                cmds::execute_print(cmds::print::PrintCommand::Mutants(args.target), Some(store))
                    .await?
            }
        },
        Commands::Init => {
            cmds::execute_init().await?;
        }
    }

    Ok(())
}
