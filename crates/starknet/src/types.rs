use serde::{Deserialize, Serialize};
use starknet_api::serde_utils::{bytes_from_hex_str, hex_str_from_bytes};

use crate::error::Error;

pub type DevnetResult<T> = Result<T, Error>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct Felt(pub(crate) [u8; 32]);

impl Felt {
    pub(crate) fn to_prefixed_hex_str(&self) -> String {
        hex_str_from_bytes::<32, true>(self.0)
    }

    pub(crate) fn to_nonprefixed_hex_str(&self) -> String {
        hex_str_from_bytes::<32, false>(self.0)
    }

    pub(crate) fn to_field_element(&self) -> DevnetResult<starknet_rs_ff::FieldElement> {
        starknet_rs_ff::FieldElement::from_bytes_be(&self.0)
            .map_err(|_| Error::ConversionError(crate::error::ConversionError::FromByteArrayError))
    }

    pub(crate) fn from_prefixed_hex_str(hex_str: &str) -> DevnetResult<Self> {
        let bytes = bytes_from_hex_str::<32, true>(hex_str)
            .map_err(|err| Error::StarknetApiError(starknet_api::StarknetApiError::InnerDeserialization(err)))?;

        Ok(Self(bytes))
    }
}

impl From<Felt> for starknet_rs_ff::FieldElement {
    fn from(value: Felt) -> Self {
        starknet_rs_ff::FieldElement::from_bytes_be(&value.0).expect("Convert Felt to FieldElement, should be the same")
    }
}

impl From<starknet_rs_ff::FieldElement> for Felt {
    fn from(value: starknet_rs_ff::FieldElement) -> Self {
        Self(value.to_bytes_be())
    }
}

impl From<u128> for Felt {
    fn from(value: u128) -> Self {
        let le_part: [u8; 16] = value.to_be_bytes();
        let byte_arr: [u8; 32] = [[0u8; 16], le_part].concat().try_into().unwrap();
        Self(byte_arr)
    }
}

impl From<starknet_api::hash::StarkFelt> for Felt {
    fn from(value: starknet_api::hash::StarkFelt) -> Self {
        let arr: [u8; 32] = value.bytes().try_into().expect("slice of incorrect length");
        Self(arr)
    }
}

impl From<Felt> for starknet_api::hash::StarkFelt {
    fn from(value: Felt) -> Self {
        starknet_api::hash::StarkFelt::new(value.0).expect("Invalid bytes")
    }
}

impl From<starknet_api::core::ClassHash> for Felt {
    fn from(value: starknet_api::core::ClassHash) -> Self {
        Felt::from(value.0)
    }
}

impl From<Felt> for starknet_api::core::ClassHash {
    fn from(value: Felt) -> Self {
        Self(starknet_api::hash::StarkFelt::from(value))
    }
}

impl From<starknet_in_rust::utils::ClassHash> for Felt {
    fn from(value: starknet_in_rust::utils::ClassHash) -> Self {
        Self(value)
    }
}

impl From<Felt> for starknet_in_rust::utils::ClassHash {
    fn from(value: Felt) -> Self {
        value.0
    }
}

impl From<starknet_api::core::PatriciaKey> for Felt {
    fn from(value: starknet_api::core::PatriciaKey) -> Self {
        let arr: [u8; 32] = value.key().bytes().try_into().expect("slice of incorrect length");
        Self(arr)
    }
}

impl TryFrom<Felt> for starknet_api::core::PatriciaKey {
    type Error = crate::error::Error;

    fn try_from(value: Felt) -> Result<Self, Self::Error> {
        Ok(starknet_api::core::PatriciaKey::try_from(starknet_api::hash::StarkFelt::from(value))?)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ContractAddress(Felt);

impl From<starknet_api::core::ContractAddress> for ContractAddress {
    fn from(value: starknet_api::core::ContractAddress) -> Self {
        Self(value.0.into())
    }
}

pub type ClassHash = Felt;
pub type Key = Felt;
pub type Balance = Felt;

#[derive(Debug, Default, Clone, Copy)]
pub struct ContractStorageKey(ContractAddress, Felt);
