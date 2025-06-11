# Frostgate ZK Backend Design

This document describes the current design of Frostgate's zero-knowledge proof backend system.

## Architecture Overview

The ZK backend system is designed with modularity, performance, and extensibility in mind. It consists of multiple layers:

```mermaid
graph TB
    subgraph "Application Layer"
        App[Application]
    end

    subgraph "ZK Backend Interface"
        ZKB[ZkBackend Trait]
        ZKBE[ZkBackendExt Trait]
    end

    subgraph "Backend Implementations"
        R0[RISC0 Backend]
        SP1[SP1 Backend]
    end

    subgraph "Caching Layer"
        CC[Circuit Cache]
        PC[Proof Cache]
    end

    subgraph "Resource Management"
        RM[Resource Monitor]
        Stats[Statistics]
    end

    App --> ZKB
    App --> ZKBE
    ZKB --> R0
    ZKB --> SP1
    R0 --> CC
    R0 --> PC
    SP1 --> CC
    SP1 --> PC
    R0 --> RM
    SP1 --> RM
    R0 --> Stats
    SP1 --> Stats
```

### 1. Core Traits and Types

- `ZkBackend`: Core trait defining the basic proving and verification interface
- `ZkBackendExt`: Extended trait for advanced features like batch operations
- Common types for configuration, statistics, and resource tracking

```mermaid
classDiagram
    class ZkBackend {
        +prove(program, input, config) ZkResult
        +verify(program, proof, config) bool
        +resource_usage() ResourceUsage
        +health_check() HealthStatus
        +stats() ZkStats
    }
    class ZkBackendExt {
        +batch_prove(programs, config) ZkResult[]
        +batch_verify(programs, proofs, config) bool[]
        +clear_cache() void
    }
    class ResourceUsage {
        +cpu_usage: f64
        +memory_usage: usize
        +active_tasks: usize
        +queue_depth: usize
    }
    class ZkStats {
        +total_proofs: u64
        +total_verifications: u64
        +avg_proving_time: Duration
    }
    ZkBackendExt --|> ZkBackend
```

### 2. Backend Implementations

```mermaid
graph LR
    subgraph "Backend Interface"
        ZB[ZkBackend]
        ZBE[ZkBackendExt]
    end

    subgraph "RISC0 Implementation"
        R0[RISC0 Backend]
        R0C[Circuit Cache]
        R0P[Proof Cache]
        R0S[Stats]
    end

    subgraph "SP1 Implementation"
        SP1[SP1 Backend]
        SP1C[Circuit Cache]
        SP1P[Proof Cache]
        SP1S[Stats]
    end

    ZB --> R0
    ZB --> SP1
    ZBE --> R0
    ZBE --> SP1
    R0 --> R0C
    R0 --> R0P
    R0 --> R0S
    SP1 --> SP1C
    SP1 --> SP1P
    SP1 --> SP1S
```

#### RISC0 Backend
- Supports message, transaction, and block verification circuits
- Implements circuit and proof caching
- Provides parallel batch operations
- Tracks resource usage and performance statistics

#### SP1 Backend
- Mirrors RISC0 functionality for compatibility
- Optimized for SP1's specific proving system
- Shares common interface and caching mechanisms

### 3. Caching System

```mermaid
graph TB
    subgraph "Circuit Cache"
        CC[LRU Cache]
        CH[SHA-256 Hash]
        CT[Compile Time]
    end

    subgraph "Proof Cache"
        PC[LRU Cache]
        PH[Program+Input Hash]
        PT[Generation Time]
    end

    subgraph "Cache Management"
        Config[Cache Config]
        Stats[Cache Stats]
        Clean[Cleanup]
    end

    CC --> CH
    CC --> CT
    PC --> PH
    PC --> PT
    Config --> CC
    Config --> PC
    CC --> Stats
    PC --> Stats
    Clean --> CC
    Clean --> PC
```

Both backends implement a two-level caching system:
- Thread-safe using RwLock
- Configurable cache sizes and TTL
- Statistics tracking
- Automatic cleanup of expired entries

### 4. Resource Management

```mermaid
graph LR
    subgraph "Resource Monitoring"
        CPU[CPU Usage]
        MEM[Memory Usage]
        Tasks[Active Tasks]
        Queue[Queue Depth]
    end

    subgraph "Health Check"
        Check[Health Status]
        Limits[Resource Limits]
    end

    CPU --> Check
    MEM --> Check
    Tasks --> Check
    Queue --> Check
    Limits --> Check
```

### 5. Circuit Types

```mermaid
graph TB
    subgraph "Circuit Types"
        M[Message 0x01]
        T[Transaction 0x02]
        B[Block 0x03]
    end

    subgraph "Message Circuit"
        M --> MH[Hash Verification]
        M --> MS[Simple Proofs]
    end

    subgraph "Transaction Circuit"
        T --> TV[Tx Validation]
        T --> TS[State Transition]
    end

    subgraph "Block Circuit"
        B --> BH[Header Validation]
        B --> BS[State Verification]
        B --> BT[Timestamp Check]
    end
```

Three standardized circuit types:
1. Message Verification (0x01)
   - Basic message and hash verification
   - Used for simple proofs

2. Transaction Verification (0x02)
   - Transaction validation
   - State transition verification

3. Block Verification (0x03)
   - Block header validation
   - Chain state verification
   - Timestamp and gas checks

## Performance Optimizations

1. Circuit Caching
   - Compiled circuits are cached
   - Significant speedup for repeated operations
   - Configurable cache size and TTL

2. Proof Caching
   - Generated proofs are cached
   - Near-instant response for identical requests
   - Automatic cache invalidation

3. Parallel Processing
   - Batch operations use rayon
   - Configurable thread pool size
   - Resource-aware scheduling

## Configuration Options

```rust
struct CacheConfig {
    max_circuits: usize,
    max_proofs: usize,
    max_age: Duration,
    enable_proof_cache: bool,
}

struct BackendOptions {
    num_threads: Option<usize>,
    memory_limit: Option<usize>,
    prover_opts: Option<ProverOpts>,
}
```

## Statistics and Monitoring

The system tracks:
- Proof generation times
- Verification times
- Cache hit rates
- Resource utilization
- Success/failure rates

## Error Handling

Comprehensive error handling for:
- Invalid inputs
- Resource exhaustion
- Circuit compilation failures
- Proof generation errors
- Verification failures

## Security Considerations

1. Input Validation
   - All inputs are validated before processing
   - Circuit-specific validation rules
   - Hash verification for integrity

2. Resource Limits
   - Configurable memory limits
   - CPU usage monitoring
   - Queue depth control

3. Cache Security
   - Automatic expiration
   - Size limits
   - No sensitive data in cache

## Future Extensions

The design allows for:
1. Additional circuit types
2. New backend implementations
3. Enhanced caching strategies
4. Advanced batching optimizations
5. Custom circuit compilation 