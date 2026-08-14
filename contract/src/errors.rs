use soroban_sdk::contracterror;

/// Enumeration of all contract-level errors.
/// Each variant maps to a unique `u32` discriminant returned to the caller.
///
/// # Cleanup note (crate reconstruction)
/// This enum previously had 91 variants, which exceeds Soroban's contract-spec XDR
/// hard cap of 50 cases for an error enum (`ScSpecUdtErrorEnumV0::cases: VecM<_, 50>`
/// in stellar-xdr 21.2.0) — past that limit `#[contracterror]` panics with
/// `LengthExceedsMax`. It was trimmed to 48 variants by removing only unreachable
/// dead code (verified with grep across the whole crate; nothing below was ever
/// constructed by any function reachable from `#[contractimpl] impl
/// StellarEscrowContract`, live or otherwise) — no surviving variant's discriminant
/// or meaning changed, and none were merged/consolidated:
///   - 16 variants that existed only for governance.rs / privacy.rs / social.rs /
///     amm.rs. None of those files are declared as `mod` anywhere in lib.rs, so they
///     were never compiled at all.
///   - 5 upgrade-system variants (UpgradeInProgress, NoUpgradeProposal,
///     UpgradeTimelockActive, NoUpgradeInProgress, RollbackWindowExpired). lib.rs's
///     own `migrate()` re-implements versioning directly against storage.rs and never
///     calls upgrade.rs; its propose/execute/run_migration/rollback/cancel functions
///     had zero callers, so they were deleted along with these errors (the
///     `UpgradeProposal` / `RollbackSnapshot` *types* are kept — still re-exported).
///   - 13 of 15 bridge-family variants (BridgeProviderNotFound,
///     BridgeProviderAlreadyRegistered, BridgeProviderLimitExceeded,
///     BridgeTradeNotFound, BridgeRetryLimitExceeded, BridgeAttestationInvalid,
///     BridgeAttestationExpired, BridgeAmountOutOfRange, BridgeChainNotSupported,
///     BridgeOracleNotAuthorized, BridgePaused, BridgeSignatureInvalid,
///     BridgeNonceAlreadyUsed). bridge.rs implemented a much larger multi-provider /
///     attestation / nonce / retry system that lib.rs's actual "Cross-Chain Bridge"
///     section never calls (it only uses storage:: directly for a simple
///     single-oracle flow) — those bridge.rs functions were deleted as dead code.
///     BridgeTradeExpired and BridgeTradeNotExpired are kept; lib.rs's simple flow
///     returns both directly.
///   - 9 variants never constructed anywhere in the crate, live or dead — evidently
///     planned but never wired up: TradeExpired, TradeNotExpired,
///     MigrationAlreadyApplied, OraclePriceInvalid, TierNotFound,
///     TemplateVersionLimitExceeded, TemplateAmountMismatch, SubscriptionExpired,
///     NoConsensus (multisig.rs never actually returns this — ties/no-majority just
///     fall through to `VotingSummary.has_consensus == false`, no error path).
///
/// Ranges (of the surviving variants; gaps are the removed ids above):
///   1–33    Core escrow / compliance / insurance / rating errors
///   34–43   Tier / template / subscription errors
///   70–73   Multi-sig arbitration errors
///   84–85   Bridge errors (simple single-oracle flow)
///   100–103 Oracle errors
///   110–111 Price-trigger errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAmount = 3,
    InvalidFeeBps = 4,
    Overflow = 5,
    Unauthorized = 6,
    ContractPaused = 7,
    InvalidStatus = 8,
    TradeNotFound = 9,
    ArbitratorNotRegistered = 10,
    KycNotVerified = 11,
    AmlNotCleared = 12,
    JurisdictionRestricted = 13,
    TradeAmountLimitExceeded = 14,
    ComplianceDataMissing = 15,
    NoFeesToWithdraw = 16,
    InvalidMetadata = 17,
    MetadataValueTooLong = 18,
    InvalidExpiry = 19,
    NoArbitrator = 20,
    MigrationVersionMismatch = 24,
    BridgeOracleNotSet = 25,
    InsuranceProviderNotRegistered = 26,
    InsurancePremiumTooHigh = 27,
    TradeNotInsured = 28,
    InsuranceAlreadyClaimed = 29,
    InsuranceClaimNotEligible = 30,
    InvalidSplitBps = 31,
    InvalidRating = 32,
    AlreadyRated = 33,
    InvalidTierConfig = 34,
    TemplateNotFound = 36,
    TemplateInactive = 37,
    TemplateNameTooLong = 38,
    SubscriptionNotFound = 41,
    SubscriptionAlreadyActive = 43,
    // Multi-sig arbitration errors (70–73)
    /// threshold == 0 or threshold > arbitrators count.
    InvalidMultiSigConfig = 70,
    /// Arbitrator has already cast a vote for this trade.
    AlreadyVoted = 71,
    /// Voting window has expired; no more votes accepted.
    VotingExpired = 72,
    /// Voting window has not yet expired; cannot force-resolve.
    VotingNotExpired = 73,
    // Bridge errors (simple single-oracle flow; 84–85)
    BridgeTradeExpired = 84,
    BridgeTradeNotExpired = 85,
    // Oracle errors
    OracleNotFound = 100,
    OracleAlreadyRegistered = 101,
    OracleListFull = 102,
    OracleUnavailable = 103,
    // Price triggers
    NoTrigger = 110,
    PriceConditionNotMet = 111,
}
