//! StellarEscrow Mobile SDK
//!
//! Provides wallet management, transaction building, offline queuing,
//! push notification registration, and signing primitives for iOS and Android.
//!
//! # Feature flags
//! - `push` — enables push notification support (default: on)

/// SDK version, kept in sync with Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod error;
pub mod flows;
pub mod mobile_error;
pub mod offline_queue;
pub mod push_notifications;
pub mod signing;
pub mod transaction_builder;
pub mod types;
pub mod wallet;
