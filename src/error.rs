//! Error types for the ZK backend interface

use std::fmt;
use thiserror::Error;

/// Result type for ZK operations
pub type ZkResult<T> = Result<T, ZkError>;

/// Error type for ZK operations
#[derive(Error, Debug)]
pub enum ZkError {
    /// Error during proof generation
    #[error("Proof generation failed: {0}")]
    ProofGeneration(String),

    /// Error during proof verification
    #[error("Proof verification failed: {0}")]
    VerificationFailed(String),

    /// Error with the program
    #[error("Program error: {0}")]
    Program(String),

    /// Error with the input
    #[error("Input error: {0}")]
    Input(String),

    /// Resource limits exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    /// Timeout occurred
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Backend-specific error
    #[error("Backend error: {0}")]
    Backend(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Error context for additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Operation being performed when error occurred
    pub operation: String,
    /// Time when error occurred
    pub timestamp: std::time::SystemTime,
    /// Additional context-specific information
    pub details: std::collections::HashMap<String, String>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation '{}' failed at {:?}", self.operation, self.timestamp)
    }
}

/// Extension trait for adding context to errors
pub trait ErrorExt<T> {
    /// Add context to an error
    fn with_context(self, operation: impl Into<String>) -> Result<T, (ZkError, ErrorContext)>;
} 