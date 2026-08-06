use soroban_sdk::{contracttype, Address, String, Vec};

pub const MAX_METADATA_SIZE: u32 = 1024;
pub const MAX_INSURANCE_PREMIUM_BPS: u32 = 1000;

// ---------------------------------------------------------------------------
// Tiers — thresholds below are inferred (no source specified a value) and
// should be reviewed as a product/business decision, not treated as final.
// Assumes 6-decimal token units (i.e. USDC-style stroops).
// ---------------------------------------------------------------------------
pub const TIER_SILVER_THRESHOLD: u64 = 10_000_000_000; // 10,000 units
pub const TIER_GOLD_THRESHOLD: u64 = 100_000_000_000; // 100,000 units

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserTier {
    Bronze,
    Silver,
    Gold,
    Custom,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserTierInfo {
    pub tier: UserTier,
    pub total_volume: u64,
    pub custom_fee_bps: Option<u32>,
}

/// Admin-configured fee schedule per tier.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TierConfig {
    pub bronze_fee_bps: u32,
    pub silver_fee_bps: u32,
    pub gold_fee_bps: u32,
}

// ---------------------------------------------------------------------------
// Subscriptions — prices/discounts below are inferred placeholders (no source
// specified values) and should be reviewed as a product/business decision.
// Duration uses the same ~5s/ledger conversion already established by
// upgrade.rs's timelock constants.
// ---------------------------------------------------------------------------
pub const SUBSCRIPTION_DURATION_LEDGERS: u32 = 518_400; // ~30 days at ~5s/ledger
pub const SUB_PRICE_BASIC: u64 = 10_000_000; // 10 units
pub const SUB_PRICE_PRO: u64 = 50_000_000; // 50 units
pub const SUB_PRICE_ENTERPRISE: u64 = 200_000_000; // 200 units
pub const SUB_DISCOUNT_BASIC_BPS: u32 = 500; // 5%
pub const SUB_DISCOUNT_PRO_BPS: u32 = 1_500; // 15%
pub const SUB_DISCOUNT_ENTERPRISE_BPS: u32 = 3_000; // 30%

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionTier {
    Basic,
    Pro,
    Enterprise,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub subscriber: Address,
    pub tier: SubscriptionTier,
    pub expires_at: u32,
    pub renewed_at: u32,
}

// ---------------------------------------------------------------------------
// Trade templates
// ---------------------------------------------------------------------------
pub const TEMPLATE_MAX_VERSIONS: u32 = 20;
pub const TEMPLATE_NAME_MAX_LEN: u32 = 128;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateTerms {
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateVersion {
    pub version: u32,
    pub terms: TemplateTerms,
    pub created_at: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TradeTemplate {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub current_version: u32,
    pub versions: Vec<TemplateVersion>,
    pub active: bool,
    pub created_at: u32,
    pub updated_at: u32,
}

// ---------------------------------------------------------------------------
// Arbitrator reputation
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitratorReputation {
    pub rating_sum: u32,
    pub rating_count: u32,
    pub resolved_count: u32,
    pub total_disputes: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionalMetadata {
    None,
    Some(String),
}

/// Volume thresholds required to reach each tier.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TierThresholds {
    /// Minimum cumulative trade volume (in token units) to reach Silver tier
    pub silver_threshold: u64,
    /// Minimum cumulative trade volume (in token units) to reach Gold tier
    pub gold_threshold: u64,
}

/// A snapshot of a user's current tier standing, including progress toward the next tier.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TierStatus {
    /// The user's current tier
    pub current_tier: UserTier,
    /// Total cumulative trading volume recorded for this user
    pub total_volume: u64,
    /// Additional volume needed to reach the next tier (0 if already at Gold or Custom)
    pub volume_to_next_tier: u64,
    /// The effective fee in basis points for the user's next trade
    pub effective_fee_bps: u32,
    /// Whether the user has a custom fee rate assigned
    pub has_custom_fee: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TradeStatus {
    Created,
    Funded,
    Completed,
    Disputed,
    Cancelled,
    AwaitingBridge, // cross-chain: waiting for bridge oracle confirmation
    BridgeFailed,   // cross-chain: bridge attestation failed
    Triggered,      // price-based trigger executed
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeResolution {
    ReleaseToBuyer,
    ReleaseToSeller,
    /// `buyer_bps`: buyer's share of the net payout, in basis points (0-10000).
    Partial { buyer_bps: u32 },
}

// ---------------------------------------------------------------------------
// Multi-signature arbitration
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiSigConfig {
    pub arbitrators: Vec<Address>,
    pub threshold: u32,
    pub voting_started_at: Option<u64>,
    pub voting_timeout_seconds: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitratorVote {
    pub arbitrator: Address,
    pub resolution: DisputeResolution,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VotingSummary {
    pub total_arbitrators: u32,
    pub votes_cast: u32,
    pub threshold: u32,
    pub has_consensus: bool,
    pub consensus_resolution: Option<DisputeResolution>,
    pub voting_expired: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArbitrationConfig {
    Single(Address),
    MultiSig(MultiSigConfig),
}

// ---------------------------------------------------------------------------
// Price Triggers
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TriggerAction {
    Cancel,
    Release,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriceTrigger {
    pub base: Address,
    pub quote: Address,
    pub target_price: i128,
    /// If true, trigger when price >= target_price. If false, trigger when price <= target_price.
    pub trigger_above: bool,
    pub action: TriggerAction,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trade {
    pub id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub amount: u64,
    pub fee: u64,
    pub arbitrator: Option<ArbitrationConfig>,
    pub status: TradeStatus,
    pub expiry_time: Option<u64>,
    pub currency: Address,
    pub metadata: OptionalMetadata,
    /// Optional price-based trigger
    pub trigger: Option<PriceTrigger>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KycStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserCompliance {
    pub kyc_status: KycStatus,
    pub aml_cleared: bool,
    pub jurisdiction: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainInfo {
    pub source_chain: String,
    pub source_tx_hash: String,
    pub expires_at_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsurancePolicy {
    pub provider: Address,
    pub premium: u64,
    pub coverage: u64,
    pub claimed: bool,
}
