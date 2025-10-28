use clap::{Parser, Subcommand};

/// MuTON - Mutation testing for TON smart contracts
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// All relative paths will be interpreted relative to this directory.
    /// All child processes will be run in this directory.
    #[arg(long, global = true)]
    pub cwd: Option<String>,

    /// Location of the muton sqlite database
    #[arg(long, global = true)]
    pub db: Option<String>,

    /// Logging level (overrides env/config). One of: trace, debug, info, warn, error
    #[arg(long, global = true)]
    pub log_level: Option<String>,

    /// Logging color control: "on" to force colors, "off" to disable; omit for auto
    #[arg(long, global = true)]
    pub log_color: Option<String>,

    /// Comma-separated list of target path globs to ignore; merged additively across sources
    #[arg(long, global = true)]
    pub ignore_targets: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new Muton workspace (config + database)
    Init,
    /// Run a mutation testing campaign
    Run(RunArgs),

    /// Generate and save mutants for a target without running tests
    Mutate(MutateArgs),

    /// Clean the database of stale targets
    Clean,

    /// Print various information about mutations and results
    Print {
        #[command(subcommand)]
        command: PrintArgs,
    },

    /// (Re-)Test a specific mutant by ID
    Test(TestArgs),

    /// Purge targets, mutants, and outcomes from the database
    Purge(PurgeArgs),
}

/// Arguments for the run command
#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Target to mutate.
    /// If a file, mutate that file.
    /// If a directory, mutate all files inside the directory.
    /// If not provided, skip mutation generation and test existing mutants without outcomes.
    #[arg(value_name = "TARGET")]
    pub target: Option<String>,

    /// Command to run the tests for the target
    #[arg(
        long,
        help = "Test command; highest non-empty source wins (CLI > env > file > default)"
    )]
    pub test_cmd: Option<String>,

    /// Timeout in seconds after which a given test run will be considered failing.
    /// Defaults to 2x the baseline test runtime.
    #[arg(long)]
    pub timeout: Option<u32>,

    /// Comma-separated list of mutation slugs to test (e.g., "ER,CR").
    /// Run `muton print mutations` for a list of slugs.
    /// Highest non-empty source wins for the whitelist (CLI > env > file > default none).
    #[arg(long)]
    pub mutations: Option<String>,

    /// Test all mutants even if more severe mutants on the same line were uncaught.
    /// By default, less severe mutants are skipped if more severe ones were uncaught.
    #[arg(long)]
    pub comprehensive: bool,

    /// Stream stdout and stderr from baseline test to stdout
    #[arg(long)]
    pub verbose: bool,
}

/// Arguments for the mutate command
#[derive(Parser, Debug)]
pub struct MutateArgs {
    /// Target to mutate.
    /// If a file, mutate that file.
    /// If a directory, mutate all files inside the directory.
    #[arg(value_name = "TARGET")]
    pub target: String,
}

/// Arguments for the list-mutations command
#[derive(Parser, Debug)]
pub struct ListMutationsArgs {
    /// Target language for mutations (defaults to "func")
    #[arg(long, default_value = "func")]
    pub language: Option<String>,
}

/// Arguments for the list-outcomes command
#[derive(Parser, Debug)]
pub struct ListOutcomesArgs {
    /// Filter outcomes by target path
    #[arg(long)]
    pub target: Option<String>,
}

/// Arguments for the print command
#[derive(Subcommand, Debug)]
pub enum PrintArgs {
    /// List all available mutations
    Mutations(PrintMutationsArgs),

    /// List outcomes of mutation tests
    Results(PrintResultsArgs),

    /// List all saved targets and their status
    Targets,

    /// print a mutant file
    Mutant(PrintMutantArgs),

    /// List all mutants or filter by target
    Mutants(PrintMutantsArgs),
}

/// Arguments for the print mutations subcommand
#[derive(Parser, Debug)]
pub struct PrintMutationsArgs {
    /// Target language for mutations (omit to show all)
    #[arg(long)]
    pub language: Option<String>,
}

/// Arguments for the print results subcommand
#[derive(Parser, Debug)]
pub struct PrintResultsArgs {
    /// Filter outcomes by target path
    #[arg(long)]
    pub target: Option<String>,

    /// Show verbose output including test output and timing information
    #[arg(long, default_value = "false")]
    pub verbose: bool,

    /// Show only the outcome for a specific mutant ID
    #[arg(long)]
    pub id: Option<i64>,

    /// Show all outcomes instead of only uncaught ones
    #[arg(long, default_value = "false")]
    pub all: bool,
}

/// Arguments for the print mutants subcommand
#[derive(Parser, Debug)]
pub struct PrintMutantArgs {
    /// Print the target file mutated by this mutant ID
    #[arg(long)]
    pub id: i64,
}

/// Arguments for the print mutants subcommand
#[derive(Parser, Debug)]
pub struct PrintMutantsArgs {
    /// Filter mutants by target path
    #[arg(long)]
    pub target: Option<String>,
}

/// Arguments for the test command
#[derive(Parser, Debug)]
pub struct TestArgs {
    /// Comma-separated list of mutation IDs to test
    #[arg(long)]
    pub ids: String,

    /// Command to run the tests for the target
    #[arg(
        long,
        help = "Test command; highest non-empty source wins (CLI > env > file > default)"
    )]
    pub test_cmd: Option<String>,

    /// Timeout in seconds after which a given test run will be considered failing.
    /// Defaults to 2x the baseline test runtime.
    #[arg(long)]
    pub timeout: Option<u32>,

    /// Stream stdout and stderr from baseline test to stdout
    #[arg(long)]
    pub verbose: bool,
}

/// Arguments for the purge command
#[derive(Parser, Debug)]
pub struct PurgeArgs {
    /// Target path to purge (if not provided, will purge all targets)
    #[arg(long)]
    pub target: Option<String>,
}
