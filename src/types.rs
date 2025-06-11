//! Core types for the ZK backend interface

use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Resource usage information for a ZK backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Current CPU usage percentage (0-100)
    pub cpu_usage: f64,
    /// Current memory usage in bytes
    pub memory_usage: usize,
    /// Number of active proving/verification tasks
    pub active_tasks: usize,
    /// Maximum number of concurrent tasks supported
    pub max_concurrent: usize,
    /// Current queue depth for pending operations
    pub queue_depth: usize,
}

/// Health status of a ZK backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Backend is fully operational
    Healthy,
    /// Backend is operational but with reduced capacity
    Degraded(String),
    /// Backend is not operational
    Unhealthy(String),
}

/// Configuration for ZK operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkConfig {
    /// Maximum size of program bytes allowed
    pub max_program_size: Option<usize>,
    /// Maximum size of input bytes allowed
    pub max_input_size: Option<usize>,
    /// Maximum time allowed for proving
    pub proving_timeout: Option<Duration>,
    /// Maximum time allowed for verification
    pub verification_timeout: Option<Duration>,
    /// Maximum memory usage allowed in bytes
    pub max_memory: Option<usize>,
}

/// Metadata about a generated proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// Time taken to generate the proof
    pub generation_time: Duration,
    /// Size of the proof in bytes
    pub proof_size: usize,
    /// Hash of the program that generated this proof
    pub program_hash: String,
    /// Timestamp when the proof was generated
    pub timestamp: std::time::SystemTime,
}

/// Statistics about proof generation/verification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZkStats {
    /// Average proving time
    pub avg_proving_time: Duration,
    /// Average verification time
    pub avg_verification_time: Duration,
    /// Total proofs generated
    pub total_proofs: usize,
    /// Total verifications performed
    pub total_verifications: usize,
    /// Total failures
    pub total_failures: usize,
} 