use std::marker::PhantomData;

use context_interface::ChainStorageConfigTr;
use primitives::{alloy_primitives::StorageValue, StorageValueTr};

/// A concrete implementation of `ChainStorageConfigTr`.
///
/// This struct is generic over `SV`, which must be a type that implements `StorageValueTr`.
/// By default, `SV` is `primitives::alloy_primitives::StorageValue` (i.e., `U256`),
/// ensuring backward compatibility and standard EVM behavior.
///
/// `EvmStorageConfig` is a zero-sized marker type; its purpose is to carry the
/// type information for `SV` into the type system.
#[derive(Default, Debug, Clone, Copy)]
pub struct EvmStorageConfig<SV: StorageValueTr = StorageValue>(PhantomData<SV>);

impl<SV: StorageValueTr> ChainStorageConfigTr for EvmStorageConfig<SV> {
    type StorageValue = SV;
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::alloy_primitives::U256; 

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct MyTestSV {
        value: U256,
        is_private_field: bool,
    }

    impl From<U256> for MyTestSV {
        fn from(u: U256) -> Self {
            MyTestSV { value: u, is_private_field: false }
        }
    }

    impl StorageValueTr for MyTestSV {
        fn to_u256(&self) -> U256 { self.value }
        fn is_zero(&self) -> bool { self.value.is_zero() } 
        fn is_private(&self) -> bool { self.is_private_field }
        fn mark_private(&mut self, is_private: bool) { self.is_private_field = is_private; }
    }

    #[test]
    fn default_evm_storage_config() {
        let config = EvmStorageConfig::default();

        let val: <EvmStorageConfig as ChainStorageConfigTr>::StorageVal = Default::default();
        assert_eq!(val.to_u256(), U256::ZERO, "Default config should use U256 as StorageVal");

        let _val_is_storage_value_type: StorageValue = val;
    }

    #[test]
    fn explicit_u256_evm_storage_config() {
        let config = EvmStorageConfig::<StorageValue>::default();
        let val: <EvmStorageConfig<StorageValue> as ChainStorageConfigTr>::StorageVal = Default::default();
        assert_eq!(val.to_u256(), U256::ZERO);
    }

    #[test]
    fn custom_sv_evm_storage_config() {
        let config = EvmStorageConfig::<MyTestSV>::default();
        let mut val: <EvmStorageConfig<MyTestSV> as ChainStorageConfigTr>::StorageVal = Default::default();

        assert_eq!(val.to_u256(), U256::ZERO);
        assert_eq!(val.is_private(), false);

        val.mark_private(true);
        assert_eq!(val.is_private(), true);
    }

    fn process_with_any_config<CSC: ChainStorageConfigTr>(_config: CSC) {
        let storage_item: CSC::StorageVal = Default::default();
        assert!(storage_item.is_zero()); 
    }

    #[test]
    fn test_generic_processing() {
        process_with_any_config(EvmStorageConfig::default()); 
        process_with_any_config(EvmStorageConfig::<MyTestSV>::default()); 
    }
}
