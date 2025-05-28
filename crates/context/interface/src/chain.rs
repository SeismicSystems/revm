use primitives::StorageValueTr;

/// Defines the configuration for the storage value type used within an EVM instance.
///
/// This trait allows different EVM setups (e.g., for different chains or custom needs)
/// to specify what concrete type represents a storage value, provided it implements
/// the `StorageValueTr` trait.
pub trait ChainStorageConfigTr {
    type StorageValue: StorageValueTr;
}
