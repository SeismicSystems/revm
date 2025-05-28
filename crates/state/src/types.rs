use super::{Account, EvmStorageSlot};
use primitives::{Address, HashMap, StorageKey, StorageValue};

/// EVM State is a mapping from addresses to accounts.
pub type EvmState<SV= StorageValue> = HashMap<Address, Account<SV>>;

/// Structure used for EIP-1153 transient storage
pub type TransientStorage<SV= StorageValue> = HashMap<(Address, StorageKey), SV>;

/// An account's Storage is a mapping from 256-bit integer keys to [EvmStorageSlot]s.
pub type EvmStorage<SV= StorageValue> = HashMap<StorageKey, EvmStorageSlot<SV>>;

