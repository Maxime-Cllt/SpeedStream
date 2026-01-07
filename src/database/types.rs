use std::fmt;

/// Trait for mapping tokio-postgres rows to domain types
///
/// This trait replaces SQLx's FromRow derive macro and provides
/// manual row-to-struct mapping for tokio-postgres.
pub trait FromPostgresRow: Sized {
    fn from_row(row: &tokio_postgres::Row) -> Result<Self, DbError>;
}

/// Unified database error type
///
/// Provides better error categorization than raw tokio_postgres::Error
/// and makes it easier to add metrics/logging per error type.
#[derive(Debug)]
pub enum DbError {
    /// Connection-related errors (connection closed, network issues, etc.)
    Connection(tokio_postgres::Error),

    /// Query execution errors (syntax errors, constraint violations, etc.)
    Query(tokio_postgres::Error),

    /// Timeout errors (connection acquisition timeout, query timeout)
    Timeout,

    /// Row parsing errors (type conversion failures, missing columns, etc.)
    RowParsing(String),

    /// Connection pool errors (pool exhaustion, configuration issues, etc.)
    PoolError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Connection(e) => write!(f, "Database connection error: {}", e),
            DbError::Query(e) => write!(f, "Database query error: {}", e),
            DbError::Timeout => write!(f, "Database operation timed out"),
            DbError::RowParsing(msg) => write!(f, "Row parsing error: {}", msg),
            DbError::PoolError(msg) => write!(f, "Connection pool error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<tokio_postgres::Error> for DbError {
    fn from(e: tokio_postgres::Error) -> Self {
        if e.is_closed() {
            DbError::Connection(e)
        } else {
            DbError::Query(e)
        }
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for DbError {
    fn from(e: bb8::RunError<tokio_postgres::Error>) -> Self {
        match e {
            bb8::RunError::User(e) => DbError::from(e),
            bb8::RunError::TimedOut => DbError::Timeout,
        }
    }
}

impl From<tokio::time::error::Elapsed> for DbError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        DbError::Timeout
    }
}
