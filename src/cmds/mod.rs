pub mod clean;
pub mod init;
pub mod mutate;
pub mod print;
pub mod purge;
pub mod run;
pub mod test;

// Re-export commands for easier access
pub use clean::execute_clean;
pub use init::execute_init;
pub use mutate::execute_mutate;
pub use print::execute_print;
pub use purge::execute_purge;
pub use run::execute_run;
pub use test::execute_test;
