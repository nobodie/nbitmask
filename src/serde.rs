use base64::{decode, encode};
use serde::de::Error;
use serde::{Deserialize, Serialize};

use crate::bit_storage::BitStorage;
use crate::error::BitMaskError;
use crate::BitMask;

///Struct used for serialization
#[derive(Clone, Debug, Serialize, Deserialize)]
struct BitMaskSerializable {
    mask: String,
    length: usize,
}

impl<T> From<&BitMask<T>> for BitMaskSerializable
where
    T: BitStorage,
{
    fn from(value: &BitMask<T>) -> Self {
        let mut bytes = Vec::new();

        value
            .mask
            .iter()
            .for_each(|e| bytes.append(&mut e.to_be_bytes()));

        Self {
            mask: encode(bytes),
            length: value.length,
        }
    }
}

impl<T> TryFrom<BitMaskSerializable> for BitMask<T>
where
    T: BitStorage,
{
    type Error = BitMaskError;

    fn try_from(value: BitMaskSerializable) -> Result<Self, Self::Error> {
        let bytes = decode(value.mask).map_err(|_| BitMaskError::DeserializationFailed)?;

        let mask: Result<Vec<T>, BitMaskError> = bytes
            .chunks(T::SIZE / 8)
            .map(|e| T::from_be_bytes(e))
            .collect();

        Ok(Self {
            mask: mask?,
            length: value.length,
        })
    }
}

impl<T> Serialize for BitMask<T>
where
    T: BitStorage + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bms = BitMaskSerializable::from(self);
        bms.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for BitMask<T>
where
    T: BitStorage,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bms = BitMaskSerializable::deserialize(deserializer)?;
        bms.try_into().map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json() {
        let mut mask: BitMask<u64> = BitMask::zeros(10);
        mask.set(0, true).unwrap();
        mask.set(3, true).unwrap();
        mask.set(8, true).unwrap();

        let json = serde_json::to_string(&mask).unwrap();

        assert_eq!(json, "{\"mask\":\"AAAAAAAAAQk=\",\"length\":10}");

        let mask2: BitMask<u64> = serde_json::from_str(&json).unwrap();

        assert_eq!(mask2, mask);
    }
}
