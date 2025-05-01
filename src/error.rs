//! This module defines the error types used in the library.

// Result type alias for the library
pub type Result<T, E = Error> = std::result::Result<T, E>;

// Error type for the library
#[derive(Debug)]
pub enum Error {
    /// Type cast error
    TypeError(String),

    WriteError(String),
    ReadError(String),
}
