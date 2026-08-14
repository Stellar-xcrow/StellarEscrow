//! Upgrade system types.
//!
//! # Reconstruction note
//! This module originally implemented a full upgrade workflow: `propose_upgrade`
//! (timelocked proposal) -> `execute_upgrade` (WASM swap + rollback snapshot)
//! -> `run_migration` (version bump) -> optional `rollback_upgrade` /
//! `cancel_upgrade`, guarded by an `UpgradeGuard` to prevent re-entrancy.
//!
//! None of those functions were ever called from `lib.rs`'s `#[contractimpl]`
//! block — `StellarEscrowContract::migrate()` re-implements its own simple
//! version bump directly against `storage::{get_version, set_version}` and
//! never calls into this module at all. With zero reachable callers, the
//! functions (and the `UpgradeGuard` type and storage-key helpers they used)
//! were deleted as dead code rather than fixed/kept. Only `UpgradeProposal`
//! and `RollbackSnapshot` remain, since `lib.rs` still re-exports them
//! (`pub use upgrade::{RollbackSnapshot, UpgradeProposal};`).

use soroban_sdk::{contracttype, Address, BytesN};

/// A pending upgrade waiting for the timelock to expire.
#[contracttype]
#[derive(Clone, Debug)]
pub struct UpgradeProposal {
    /// WASM hash of the new contract code.
    pub new_wasm_hash: BytesN<32>,
    /// Ledger sequence after which the upgrade may be executed.
    pub executable_after: u32,
    /// Address that submitted the proposal.
    pub proposed_by: Address,
    /// Human-readable description / changelog (max 256 chars enforced off-chain).
    pub description: soroban_sdk::String,
}

/// Snapshot of critical instance-storage fields taken just before WASM swap.
/// Used to restore state on rollback.
#[contracttype]
#[derive(Clone, Debug)]
pub struct RollbackSnapshot {
    /// Contract version before the upgrade.
    pub version_before: u32,
    /// Platform fee bps before the upgrade.
    pub fee_bps_before: u32,
    /// Ledger sequence at which the snapshot was taken.
    pub snapshot_ledger: u32,
    /// Last ledger at which a rollback is still permitted.
    pub rollback_deadline: u32,
}
