//! Cross-chain bridge integration for StellarEscrow.
//!
//! # Reconstruction note
//! This module originally implemented a much larger multi-provider /
//! attestation / nonce / retry bridge system: `register_bridge_provider`,
//! `get_bridge_provider(s)`, `deactivate_bridge_provider`,
//! `create_cross_chain_trade`, `get_cross_chain_trade`,
//! `update_attestation_status`, `validate_bridge_attestation`,
//! `pause_bridge`/`resume_bridge`/`is_bridge_paused`, `get_next_bridge_nonce`,
//! `retry_bridge_attestation`, `rollback_cross_chain_trade`, plus the
//! `BridgeProviderConfig` type and their storage-key helpers/constants.
//!
//! None of it was ever called from `lib.rs`'s `#[contractimpl]` block. The
//! contract's actual "Cross-Chain Bridge" section (see
//! `StellarEscrowContract::{set_bridge_oracle, create_cross_chain_trade,
//! confirm_bridge_deposit, expire_bridge_trade, get_cross_chain_info}` in
//! lib.rs) implements a simpler single-oracle flow directly against
//! `storage.rs` / `types::CrossChainInfo`, and only ever reaches this module
//! via `pub use bridge::{BridgeAttestation, BridgeProvider, BridgeValidation,
//! CrossChainTrade};` for the type re-exports. That entire unreachable
//! subsystem — and the bugs in it (`.iter_mut()` on `soroban_sdk::Vec`, which
//! has no such method; an `if let Ok(_) = ...get::<_, _>(..)` where `.get()`
//! actually returns `Option`, not `Result`) — was deleted as dead code rather
//! than fixed. Only the types actually re-exported/used by `lib.rs` remain
//! below (`AttestationStatus` is kept too, as a field type of `CrossChainTrade`).

use soroban_sdk::{contracttype, Address, Bytes, String};

/// Supported bridge protocols
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BridgeProvider {
    Wormhole,
    IBC,
    Axelar,
    LayerZero,
    Custom(String), // For extensibility
}

/// Bridge attestation status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttestationStatus {
    Pending,
    Confirmed,
    Failed,
    Expired,
}

/// Cross-chain trade metadata
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainTrade {
    /// Trade ID on Stellar (destination chain)
    pub trade_id: u64,
    /// Source blockchain identifier (e.g., "ethereum", "polygon", "avalanche")
    pub source_chain: String,
    /// Destination chain (always "stellar" for this contract)
    pub dest_chain: String,
    /// Source chain transaction hash
    pub source_tx_hash: String,
    /// Bridge provider used
    pub bridge_provider: BridgeProvider,
    /// Attestation ID from bridge
    pub attestation_id: String,
    /// Current attestation status
    pub attestation_status: AttestationStatus,
    /// Timestamp when attestation was submitted
    pub attestation_timestamp: u64,
    /// Number of retry attempts
    pub retry_count: u32,
    /// Minimum block confirmations required on source chain
    pub min_confirmations: u32,
    /// Current block confirmations on source chain
    pub current_confirmations: u32,
    /// Bridge fee paid (in stroops)
    pub bridge_fee: u64,
    /// Metadata about the bridge transaction
    pub bridge_metadata: Option<String>,
}

/// Bridge attestation from oracle
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAttestation {
    pub attestation_id: String,
    pub trade_id: u64,
    pub source_chain: String,
    pub source_tx_hash: String,
    pub amount: u64,
    pub recipient: Address,
    pub timestamp: u64,
    /// `Bytes`, not `Vec<u8>`: `u8` does not implement soroban_sdk's
    /// `TryFromVal`, so `soroban_sdk::Vec<u8>` cannot derive
    /// `Debug`/`Eq`/`PartialEq` (nor support `==`) — this was one of the
    /// original build errors. `Bytes` is the SDK's native byte-string SCVal
    /// type and supports all of these directly.
    pub signature: Bytes,
    pub provider: BridgeProvider,
}

/// Bridge validation result
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeValidation {
    pub valid: bool,
    pub error_code: Option<u32>,
    pub error_message: Option<String>,
    pub confirmations: u32,
}
