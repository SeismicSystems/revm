use primitives::{HashMap, StorageKey, StorageValue, StorageValueTr};
use state::{AccountInfo, EvmStorageSlot};

// Plain account of StateDatabase.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlainAccount<SV: StorageValueTr= StorageValue> {
    pub info: AccountInfo,
    pub storage: PlainStorage<SV>,
}

impl<SV: StorageValueTr> PlainAccount<SV> {
    pub fn new_empty_with_storage(storage: PlainStorage<SV>) -> Self {
        Self {
            info: AccountInfo::default(),
            storage,
        }
    }

    pub fn into_components(self) -> (AccountInfo, PlainStorage<SV>) {
        (self.info, self.storage)
    }
}

/// This type keeps track of the current value of a storage slot.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(
    serialize = "SV: serde::Serialize",
    deserialize = "SV: serde::de::DeserializeOwned" 
)))]
pub struct StorageSlot<SV: StorageValueTr = StorageValue> {
    /// The value of the storage slot before it was changed.
    ///
    /// When the slot is first loaded, this is the original value.
    ///
    /// If the slot was not changed, this is equal to the present value.
    pub previous_or_original_value: SV,
    /// When loaded with sload present value is set to original value
    pub present_value: SV,
}

impl<SV: StorageValueTr> From<EvmStorageSlot<SV>> for StorageSlot<SV> {
    fn from(value: EvmStorageSlot<SV>) -> Self {
        Self::new_changed(value.original_value, value.present_value)
    }
}

impl<SV: StorageValueTr> StorageSlot<SV> {
    /// Creates a new _unchanged_ `StorageSlot` for the given value.
    pub fn new(original: SV) -> Self {
        Self {
            previous_or_original_value: original,
            present_value: original,
        }
    }

    /// Creates a new _changed_ `StorageSlot`.
    pub fn new_changed(
        previous_or_original_value: SV,
        present_value: SV,
    ) -> Self {
        Self {
            previous_or_original_value,
            present_value,
        }
    }

    /// Returns true if the present value differs from the original value
    pub fn is_changed(&self) -> bool {
        self.previous_or_original_value != self.present_value
    }

    /// Returns the original value of the storage slot.
    pub fn original_value(&self) -> SV {
        self.previous_or_original_value
    }

    /// Returns the current value of the storage slot.
    pub fn present_value(&self) -> SV {
        self.present_value
    }
}

/// This storage represent values that are before block changed.
///
/// Note: Storage that we get EVM contains original values before block changed.
pub type StorageWithOriginalValues<SV = StorageValue> = HashMap<StorageKey, StorageSlot<SV>>;

/// Simple plain storage that does not have previous value.
/// This is used for loading from database, cache and for bundle state.
pub type PlainStorage<SV = StorageValue> = HashMap<StorageKey, SV>;

impl From<AccountInfo> for PlainAccount<StorageValue> {
    fn from(info: AccountInfo) -> Self {
        Self {
            info,
            storage: PlainStorage::<StorageValue>::default(),
        }
    }
}
