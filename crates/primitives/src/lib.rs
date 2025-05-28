//! # revm-primitives
//!
//! EVM primitive types.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub mod constants;
pub mod eip170;
pub mod eip4844;
pub mod eip7702;
pub mod eip7823;
pub mod eof;
pub mod hardfork;

use core::fmt::Debug;
use core::hash::Hash;

pub use constants::*;

// Reexport alloy primitives.

pub use alloy_primitives::map::{self, hash_map, hash_set, HashMap, HashSet};
pub use alloy_primitives::{
    self, address, b256, bytes, fixed_bytes, hex, hex_literal, keccak256, ruint, uint, Address,
    Bytes, FixedBytes, Log, LogData, TxKind, B256, I128, I256, U128, U256,
};

/// type alias for storage keys
pub type StorageKey = U256;
/// type alias for storage values
pub type StorageValue = U256;

/// Trait defining the behavior for types that can be used as storage values within REVM.
pub trait StorageValueTr:
    Default
    + Clone
    + Copy
    + Send
    + Sync
    + Debug
    + PartialEq
    + Eq
    + Hash
    + Ord
    + From<U256> 
    + serde::Serialize
    + serde::de::DeserializeOwned
{
    /// Converts the storage value to its U256 representation.
    fn to_u256(&self) -> U256;

    /// Checks if the storage value represents a zero state.
    fn is_zero(&self) -> bool;

    /// Checks if the storage slot is marked as private.
    fn is_private(&self) -> bool { false } // Default for U256

    /// Marks the storage slot with a given privacy status.
    fn mark_private(&mut self, _is_private: bool) { /* no-op for U256 */ }
}

impl StorageValueTr for U256 {
    #[inline]
    fn to_u256(&self) -> U256 { *self }

    #[inline]
    fn is_zero(&self) -> bool { self == &U256::ZERO }
}
