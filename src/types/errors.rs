use std::io;
use thiserror::Error;

use crate::types::HashError;

#[derive(Error, Debug)]
pub enum MutonError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Store error: {0}")]
    Store(#[from] StoreError),
    #[error("{0}")]
    Custom(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Target not found: {0}")]
    TargetNotFound(String),
    // Add any other error types you need here
}

pub type MutonResult<T> = Result<T, MutonError>;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("No entry found by id: {0}")]
    NotFound(i64),
    #[error("Invalid hash: {0}")]
    InvalidHash(#[from] HashError),
    #[error("Invalid status: {0}")]
    InvalidStatus(String),
    #[error("Invalid time: {0}")]
    InvalidTime(#[from] chrono::ParseError),
    #[error("Invalid target: {0}")]
    InvalidTarget(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

pub type StoreResult<T> = std::result::Result<T, StoreError>;
