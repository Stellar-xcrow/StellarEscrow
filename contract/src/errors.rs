use soroban_sdk::contracterror;

/// Enumeration of all contract-level errors.
/// Each variant maps to a unique `u32` discriminant returned to the caller.
/// Ranges:
///   1–39    Core escrow / tier / template / subscription / governance / privacy errors
///   40–44   Oracle errors
///   50–54   AMM errors (amm.rs is not currently wired into the contract)
///   60–66   Upgrade system errors
///   70–74   Multi-sig arbitration errors
///   80–94   Bridge errors
///   110–119 Price-trigger errors
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
    TradeExpired = 21,
    TradeNotExpired = 22,
    MigrationAlreadyApplied = 23,
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
    // Tiers / templates / subscriptions (not all currently exposed as contract
    // entry points — see reconstruction notes)
    InvalidTierConfig = 34,
    TierNotFound = 35,
    TemplateNotFound = 36,
    TemplateInactive = 37,
    TemplateNameTooLong = 38,
    TemplateVersionLimitExceeded = 39,
    TemplateAmountMismatch = 40,
    SubscriptionNotFound = 41,
    SubscriptionExpired = 42,
    SubscriptionAlreadyActive = 43,
    // Governance (governance.rs is not currently wired into the contract)
    ProposalNotFound = 44,
    ProposalNotActive = 45,
    InsufficientVotingPower = 46,
    ProposalNotPassed = 47,
    ProposalAlreadyExecuted = 48,
    VotingEnded = 49,
    // Privacy (privacy.rs is not currently wired into the contract)
    PrivacyDataTooLong = 50,
    DisclosureGrantNotFound = 51,
    DisclosureUnauthorized = 52,
    // Social (social.rs is not currently wired into the contract)
    CannotFollowSelf = 53,
    NotFollowing = 54,
    // Oracle errors (40–44 in the doc comment above collide with the core
    // range that grew past 39; kept in a dedicated 100s block instead)
    OracleNotFound = 100,
    OracleAlreadyRegistered = 101,
    OracleListFull = 102,
    OracleUnavailable = 103,
    OraclePriceInvalid = 104,
    // AMM errors (amm.rs is not currently wired into the contract)
    AmmPoolNotFound = 120,
    AmmSlippageExceeded = 121,
    AmmInsufficientShares = 122,
    AmmInvalidPair = 123,
    AmmPoolAlreadyExists = 124,
    // Upgrade system errors
    /// An upgrade is already in progress (guard is set).
    UpgradeInProgress = 60,
    /// No upgrade proposal exists to execute or cancel.
    NoUpgradeProposal = 61,
    /// Timelock has not yet expired; upgrade cannot be executed yet.
    UpgradeTimelockActive = 62,
    /// No upgrade guard is set; migrate/rollback called out of sequence.
    NoUpgradeInProgress = 63,
    /// Rollback window has passed; state cannot be reverted automatically.
    RollbackWindowExpired = 64,
    // Multi-sig arbitration errors (70–74)
    /// threshold == 0 or threshold > arbitrators count.
    InvalidMultiSigConfig = 70,
    /// Arbitrator has already cast a vote for this trade.
    AlreadyVoted = 71,
    /// Voting window has expired; no more votes accepted.
    VotingExpired = 72,
    /// Voting window has not yet expired; cannot force-resolve.
    VotingNotExpired = 73,
    /// No consensus reached among arbitrators.
    NoConsensus = 74,
    // Bridge errors (80–94)
    BridgeProviderNotFound = 80,
    BridgeProviderAlreadyRegistered = 81,
    BridgeProviderLimitExceeded = 82,
    BridgeTradeNotFound = 83,
    BridgeTradeExpired = 84,
    BridgeTradeNotExpired = 85,
    BridgeRetryLimitExceeded = 86,
    BridgeAttestationInvalid = 87,
    BridgeAttestationExpired = 88,
    BridgeAmountOutOfRange = 89,
    BridgeChainNotSupported = 90,
    BridgeOracleNotAuthorized = 91,
    BridgePaused = 92,
    BridgeSignatureInvalid = 93,
    BridgeNonceAlreadyUsed = 94,
    // Price triggers
    NoTrigger = 110,
    PriceConditionNotMet = 111,
}
