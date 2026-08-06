use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol, Vec};

use crate::errors::ContractError;
use crate::types::{
    ArbitratorReputation, ArbitratorVote, CrossChainInfo, InsurancePolicy, Subscription,
    TierConfig, Trade, TradeTemplate, UserCompliance, UserTierInfo,
};

// ---------------------------------------------------------------------------
// Instance storage keys — a single DataKey enum for straightforward
// single-value / single-address-indexed fields.
// ---------------------------------------------------------------------------
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    UsdcToken,
    FeeBps,
    Initialized,
    Paused,
    TradeCounter,
    AccumulatedFees,
    Version,
    GlobalTradeLimit,
    BridgeOracle,
    Trade(u64),
    Arbitrator(Address),
    UserCompliance(Address),
    UserTradeLimit(Address),
    JurisdictionRule(String),
    CrossChainInfo(u64),
    InsuranceProvider(Address),
    InsurancePolicy(u64),
}

// ---------------------------------------------------------------------------
// Persistent storage key prefixes for fields without a DataKey variant —
// single-char strings minimise key size, directly reducing ledger entry
// bytes and CPU cost.
// ---------------------------------------------------------------------------
const CURRENCY_FEES_PREFIX: &str = "CF";
const USER_TIER_PREFIX: &str = "U";
const TEMPLATE_PREFIX: &str = "P";
const SUBSCRIPTION_PREFIX: &str = "SB";
const ARB_FEE_PREFIX: &str = "AF";
const ARB_LIST_KEY: &str = "ARBLIST";
const MULTISIG_VOTE_PREFIX: &str = "MV";
const ARB_REP_PREFIX: &str = "AR";
const ARB_RATED_PREFIX: &str = "RT";

fn key_tier_cfg() -> Symbol {
    symbol_short!("TIER_CFG")
}
fn key_tmpl_ctr() -> Symbol {
    symbol_short!("TMPL_CTR")
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Initialized).unwrap_or(false)
}

pub fn set_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
}

// ---------------------------------------------------------------------------
// Admin
// ---------------------------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

// ---------------------------------------------------------------------------
// USDC Token
// ---------------------------------------------------------------------------

pub fn set_usdc_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::UsdcToken, token);
}

pub fn get_usdc_token(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::UsdcToken)
        .ok_or(ContractError::NotInitialized)
}

// ---------------------------------------------------------------------------
// Fee BPS
// ---------------------------------------------------------------------------

pub fn set_fee_bps(env: &Env, fee_bps: u32) {
    env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
}

pub fn get_fee_bps(env: &Env) -> Result<u32, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::FeeBps)
        .ok_or(ContractError::NotInitialized)
}

// ---------------------------------------------------------------------------
// Trade Counter
// ---------------------------------------------------------------------------

pub fn set_trade_counter(env: &Env, counter: u64) {
    env.storage().instance().set(&DataKey::TradeCounter, &counter);
}

pub fn get_trade_counter(env: &Env) -> Result<u64, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::TradeCounter)
        .ok_or(ContractError::NotInitialized)
}

pub fn increment_trade_counter(env: &Env) -> Result<u64, ContractError> {
    let next = env
        .storage()
        .instance()
        .get::<_, u64>(&DataKey::TradeCounter)
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(ContractError::Overflow)?;
    env.storage().instance().set(&DataKey::TradeCounter, &next);
    Ok(next)
}

// ---------------------------------------------------------------------------
// Accumulated Fees (legacy single-currency path)
// ---------------------------------------------------------------------------

pub fn set_accumulated_fees(env: &Env, fees: u64) {
    env.storage().instance().set(&DataKey::AccumulatedFees, &fees);
}

pub fn get_accumulated_fees(env: &Env) -> Result<u64, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::AccumulatedFees)
        .ok_or(ContractError::NotInitialized)
}

/// Atomically add `delta` to the legacy accumulated-fees counter in a single
/// read-modify-write, avoiding a separate get + set pair. Returns the new total.
pub fn add_accumulated_fees(env: &Env, delta: u64) -> Result<u64, ContractError> {
    let updated = env
        .storage()
        .instance()
        .get::<_, u64>(&DataKey::AccumulatedFees)
        .unwrap_or(0)
        .checked_add(delta)
        .ok_or(ContractError::Overflow)?;
    env.storage().instance().set(&DataKey::AccumulatedFees, &updated);
    Ok(updated)
}

// ---------------------------------------------------------------------------
// Per-currency accumulated fees
// ---------------------------------------------------------------------------

pub fn get_currency_fees(env: &Env, currency: &Address) -> u64 {
    let key = (CURRENCY_FEES_PREFIX, currency);
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_currency_fees(env: &Env, currency: &Address, fees: u64) {
    let key = (CURRENCY_FEES_PREFIX, currency);
    env.storage().persistent().set(&key, &fees);
}

/// Atomically add `delta` to per-currency fees in a single read-modify-write,
/// avoiding a separate `get_currency_fees` + `set_currency_fees` pair.
pub fn add_currency_fees(env: &Env, currency: &Address, delta: u64) -> Result<(), ContractError> {
    let key = (CURRENCY_FEES_PREFIX, currency);
    let current: u64 = env.storage().persistent().get(&key).unwrap_or(0);
    let new_fees = current.checked_add(delta).ok_or(ContractError::Overflow)?;
    env.storage().persistent().set(&key, &new_fees);
    Ok(())
}

// ---------------------------------------------------------------------------
// Trades
// ---------------------------------------------------------------------------

pub fn save_trade(env: &Env, trade_id: u64, trade: &Trade) {
    env.storage().persistent().set(&DataKey::Trade(trade_id), trade);
}

pub fn get_trade(env: &Env, trade_id: u64) -> Result<Trade, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Trade(trade_id))
        .ok_or(ContractError::TradeNotFound)
}

// ---------------------------------------------------------------------------
// Arbitrators (registration flag)
// ---------------------------------------------------------------------------

pub fn save_arbitrator(env: &Env, arbitrator: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::Arbitrator(arbitrator.clone()), &true);
}

pub fn remove_arbitrator(env: &Env, arbitrator: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Arbitrator(arbitrator.clone()));
}

pub fn has_arbitrator(env: &Env, arbitrator: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Arbitrator(arbitrator.clone()))
}

// ---------------------------------------------------------------------------
// Arbitrator self-registration with fee (Issue #120)
// ---------------------------------------------------------------------------

/// Store an arbitrator's self-declared service fee (i128 stroops).
pub fn save_arbitrator_fee(env: &Env, arbitrator: &Address, fee: i128) {
    let key = (ARB_FEE_PREFIX, arbitrator);
    env.storage().persistent().set(&key, &fee);
}

/// Remove an arbitrator's fee entry on deregistration.
pub fn remove_arbitrator_fee(env: &Env, arbitrator: &Address) {
    let key = (ARB_FEE_PREFIX, arbitrator);
    env.storage().persistent().remove(&key);
}

/// Return the stored fee for a registered arbitrator, or 0 if not found.
pub fn get_arbitrator_fee(env: &Env, arbitrator: &Address) -> i128 {
    let key = (ARB_FEE_PREFIX, arbitrator);
    env.storage().persistent().get(&key).unwrap_or(0)
}

/// Append `arbitrator` to the persistent registry list (no-op if already present).
pub fn add_to_arbitrator_list(env: &Env, arbitrator: &Address) {
    let mut list: Vec<Address> = env
        .storage()
        .persistent()
        .get(&ARB_LIST_KEY)
        .unwrap_or_else(|| Vec::new(env));
    for i in 0..list.len() {
        if list.get(i).unwrap() == *arbitrator {
            return;
        }
    }
    list.push_back(arbitrator.clone());
    env.storage().persistent().set(&ARB_LIST_KEY, &list);
}

/// Remove `arbitrator` from the persistent registry list.
pub fn remove_from_arbitrator_list(env: &Env, arbitrator: &Address) {
    let list: Vec<Address> = env
        .storage()
        .persistent()
        .get(&ARB_LIST_KEY)
        .unwrap_or_else(|| Vec::new(env));
    let mut new_list = Vec::new(env);
    for i in 0..list.len() {
        let a = list.get(i).unwrap();
        if a != *arbitrator {
            new_list.push_back(a);
        }
    }
    env.storage().persistent().set(&ARB_LIST_KEY, &new_list);
}

/// Return the full list of registered arbitrators.
pub fn get_arbitrator_list(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&ARB_LIST_KEY)
        .unwrap_or_else(|| Vec::new(env))
}

// ---------------------------------------------------------------------------
// Multi-Signature Arbitration Votes
// ---------------------------------------------------------------------------

pub fn save_arbitrator_vote(env: &Env, trade_id: u64, arbitrator: &Address, vote: &ArbitratorVote) {
    let key = (MULTISIG_VOTE_PREFIX, trade_id, arbitrator);
    env.storage().persistent().set(&key, vote);
}

pub fn get_arbitrator_vote(env: &Env, trade_id: u64, arbitrator: &Address) -> Option<ArbitratorVote> {
    let key = (MULTISIG_VOTE_PREFIX, trade_id, arbitrator);
    env.storage().persistent().get(&key)
}

pub fn has_arbitrator_voted(env: &Env, trade_id: u64, arbitrator: &Address) -> bool {
    let key = (MULTISIG_VOTE_PREFIX, trade_id, arbitrator);
    env.storage().persistent().has(&key)
}

/// Collect all votes cast for `trade_id` across the given arbitrator list.
/// Iterates only the arbitrators that are part of this trade to avoid
/// unnecessary storage reads.
pub fn get_all_votes_for_trade(env: &Env, trade_id: u64, arbitrators: &Vec<Address>) -> Vec<ArbitratorVote> {
    let mut votes = Vec::new(env);
    for i in 0..arbitrators.len() {
        let arbitrator = arbitrators.get(i).unwrap();
        if let Some(vote) = get_arbitrator_vote(env, trade_id, &arbitrator) {
            votes.push_back(vote);
        }
    }
    votes
}

pub fn clear_votes_for_trade(env: &Env, trade_id: u64, arbitrators: &Vec<Address>) {
    for i in 0..arbitrators.len() {
        let arbitrator = arbitrators.get(i).unwrap();
        let key = (MULTISIG_VOTE_PREFIX, trade_id, &arbitrator);
        env.storage().persistent().remove(&key);
    }
}

// ---------------------------------------------------------------------------
// Pause State
// ---------------------------------------------------------------------------

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tier Config
// ---------------------------------------------------------------------------

pub fn save_tier_config(env: &Env, config: &TierConfig) {
    env.storage().instance().set(&key_tier_cfg(), config);
}

pub fn get_tier_config(env: &Env) -> Option<TierConfig> {
    env.storage().instance().get(&key_tier_cfg())
}

// ---------------------------------------------------------------------------
// Per-User Tier
// ---------------------------------------------------------------------------

pub fn save_user_tier(env: &Env, user: &Address, info: &UserTierInfo) {
    let key = (USER_TIER_PREFIX, user);
    env.storage().persistent().set(&key, info);
}

pub fn get_user_tier(env: &Env, user: &Address) -> Option<UserTierInfo> {
    let key = (USER_TIER_PREFIX, user);
    env.storage().persistent().get(&key)
}

// ---------------------------------------------------------------------------
// User Compliance
// ---------------------------------------------------------------------------

pub fn save_user_compliance(env: &Env, user: &Address, compliance: &UserCompliance) {
    env.storage()
        .persistent()
        .set(&DataKey::UserCompliance(user.clone()), compliance);
}

pub fn get_user_compliance(env: &Env, user: &Address) -> Option<UserCompliance> {
    env.storage()
        .persistent()
        .get(&DataKey::UserCompliance(user.clone()))
}

pub fn set_user_trade_limit(env: &Env, user: &Address, limit: u64) {
    env.storage()
        .persistent()
        .set(&DataKey::UserTradeLimit(user.clone()), &limit);
}

pub fn get_user_trade_limit(env: &Env, user: &Address) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::UserTradeLimit(user.clone()))
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Jurisdiction Restrictions
// ---------------------------------------------------------------------------

pub fn set_jurisdiction_rule(env: &Env, jurisdiction: &String, allowed: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::JurisdictionRule(jurisdiction.clone()), &allowed);
}

pub fn is_jurisdiction_allowed(env: &Env, jurisdiction: &String) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::JurisdictionRule(jurisdiction.clone()))
        .unwrap_or(true)
}

// ---------------------------------------------------------------------------
// Global Trade Limit
// ---------------------------------------------------------------------------

pub fn set_global_trade_limit(env: &Env, limit: u64) {
    env.storage().instance().set(&DataKey::GlobalTradeLimit, &limit);
}

pub fn get_global_trade_limit(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::GlobalTradeLimit)
        .unwrap_or(u64::MAX)
}

// ---------------------------------------------------------------------------
// Trade Templates
// ---------------------------------------------------------------------------

pub fn get_template_counter(env: &Env) -> u64 {
    env.storage().instance().get(&key_tmpl_ctr()).unwrap_or(0)
}

pub fn increment_template_counter(env: &Env) -> Result<u64, ContractError> {
    let next = get_template_counter(env)
        .checked_add(1)
        .ok_or(ContractError::Overflow)?;
    env.storage().instance().set(&key_tmpl_ctr(), &next);
    Ok(next)
}

pub fn save_template(env: &Env, template_id: u64, template: &TradeTemplate) {
    let key = (TEMPLATE_PREFIX, template_id);
    env.storage().persistent().set(&key, template);
}

pub fn get_template(env: &Env, template_id: u64) -> Result<TradeTemplate, ContractError> {
    let key = (TEMPLATE_PREFIX, template_id);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::TemplateNotFound)
}

// ---------------------------------------------------------------------------
// Subscriptions
// ---------------------------------------------------------------------------

pub fn save_subscription(env: &Env, subscriber: &Address, sub: &Subscription) {
    let key = (SUBSCRIPTION_PREFIX, subscriber);
    env.storage().persistent().set(&key, sub);
}

pub fn get_subscription(env: &Env, subscriber: &Address) -> Option<Subscription> {
    let key = (SUBSCRIPTION_PREFIX, subscriber);
    env.storage().persistent().get(&key)
}

pub fn remove_subscription(env: &Env, subscriber: &Address) {
    let key = (SUBSCRIPTION_PREFIX, subscriber);
    env.storage().persistent().remove(&key);
}

// ---------------------------------------------------------------------------
// Arbitrator Reputation
// ---------------------------------------------------------------------------

pub fn get_arbitrator_reputation(env: &Env, arbitrator: &Address) -> ArbitratorReputation {
    let key = (ARB_REP_PREFIX, arbitrator);
    env.storage().persistent().get(&key).unwrap_or(ArbitratorReputation {
        rating_sum: 0,
        rating_count: 0,
        resolved_count: 0,
        total_disputes: 0,
    })
}

pub fn save_arbitrator_reputation(env: &Env, arbitrator: &Address, rep: &ArbitratorReputation) {
    let key = (ARB_REP_PREFIX, arbitrator);
    env.storage().persistent().set(&key, rep);
}

pub fn has_rated(env: &Env, trade_id: u64, rater: &Address) -> bool {
    let key = (ARB_RATED_PREFIX, trade_id, rater);
    env.storage().persistent().has(&key)
}

pub fn mark_rated(env: &Env, trade_id: u64, rater: &Address) {
    let key = (ARB_RATED_PREFIX, trade_id, rater);
    env.storage().persistent().set(&key, &true);
}

// ---------------------------------------------------------------------------
// Contract Version
// ---------------------------------------------------------------------------

pub fn set_version(env: &Env, version: u32) {
    env.storage().instance().set(&DataKey::Version, &version);
}

pub fn get_version(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::Version).unwrap_or(1)
}

// ---------------------------------------------------------------------------
// Bridge Oracle (simple single-oracle cross-chain flow)
// ---------------------------------------------------------------------------

pub fn set_bridge_oracle(env: &Env, oracle: &Address) {
    env.storage().instance().set(&DataKey::BridgeOracle, oracle);
}

pub fn get_bridge_oracle(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::BridgeOracle)
}

// ---------------------------------------------------------------------------
// Cross-Chain Info (simple single-oracle cross-chain flow)
// ---------------------------------------------------------------------------

pub fn save_cross_chain_info(env: &Env, trade_id: u64, info: &CrossChainInfo) {
    env.storage()
        .persistent()
        .set(&DataKey::CrossChainInfo(trade_id), info);
}

pub fn get_cross_chain_info(env: &Env, trade_id: u64) -> Option<CrossChainInfo> {
    env.storage()
        .persistent()
        .get(&DataKey::CrossChainInfo(trade_id))
}

// ---------------------------------------------------------------------------
// Insurance Providers
// ---------------------------------------------------------------------------

pub fn save_insurance_provider(env: &Env, provider: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::InsuranceProvider(provider.clone()), &true);
}

pub fn remove_insurance_provider(env: &Env, provider: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::InsuranceProvider(provider.clone()));
}

pub fn has_insurance_provider(env: &Env, provider: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::InsuranceProvider(provider.clone()))
}

// ---------------------------------------------------------------------------
// Insurance Policies
// ---------------------------------------------------------------------------

pub fn save_insurance_policy(env: &Env, trade_id: u64, policy: &InsurancePolicy) {
    env.storage()
        .persistent()
        .set(&DataKey::InsurancePolicy(trade_id), policy);
}

pub fn get_insurance_policy(env: &Env, trade_id: u64) -> Option<InsurancePolicy> {
    env.storage()
        .persistent()
        .get(&DataKey::InsurancePolicy(trade_id))
}
