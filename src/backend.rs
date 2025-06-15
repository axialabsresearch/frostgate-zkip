#![allow(unused_imports)]
#![allow(unused_variables)]

//! Core ZK backend trait and associated types

use async_trait::async_trait;
use std::fmt::Debug;

use crate::{
    error::{ZkError, ZkResult},
    types::{HealthStatus, ProofMetadata, ResourceUsage, ZkConfig, ZkStats},
};

/// Core trait for ZK backend implementations
#[async_trait]
pub trait ZkBackend: Send + Sync + Debug {
    /// Generate a proof for the given program and input
    async fn prove(
        &self,
        program: &[u8],
        input: &[u8],
        config: Option<&ZkConfig>,
    ) -> ZkResult<(Vec<u8>, ProofMetadata)>;

    /// Verify a proof against a program
    async fn verify(
        &self,
        program: &[u8],
        proof: &[u8],
        config: Option<&ZkConfig>,
    ) -> ZkResult<bool>;

    /// Get current resource usage
    fn resource_usage(&self) -> ResourceUsage;

    /// Get current health status
    async fn health_check(&self) -> HealthStatus {
        HealthStatus::Healthy
    }

    /// Get statistics about proof generation/verification
    fn stats(&self) -> ZkStats {
        ZkStats::default()
    }

    /// Initialize the backend with optional configuration
    async fn initialize(&mut self, config: Option<&ZkConfig>) -> ZkResult<()> {
        Ok(())
    }

    /// Shutdown the backend and cleanup resources
    async fn shutdown(&mut self) -> ZkResult<()> {
        Ok(())
    }
}

/// Extension trait for advanced ZK backend features
#[async_trait]
pub trait ZkBackendExt: ZkBackend {
    /// Generate proofs for multiple programs in batch
    async fn batch_prove(
        &self,
        programs: &[(&[u8], &[u8])],
        config: Option<&ZkConfig>,
    ) -> ZkResult<Vec<(Vec<u8>, ProofMetadata)>>;

    /// Verify multiple proofs in batch
    async fn batch_verify(
        &self,
        verifications: &[(&[u8], &[u8])],
        config: Option<&ZkConfig>,
    ) -> ZkResult<Vec<bool>>;

    /// Clear any cached data
    async fn clear_cache(&mut self) -> ZkResult<()>;

    /// Get backend-specific capabilities
    fn capabilities(&self) -> Vec<String>;
} 