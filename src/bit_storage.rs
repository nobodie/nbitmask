use crate::error::BitMaskError;

///The trait required for any Container
pub trait BitStorage {
    ///Number of bits stored within this container
    const SIZE: usize;
    ///Value representing a 0
    const ZERO: Self;
    ///Value representing a 1
    const ONE: Self;

    ///Returns the number of ones in the binary representation of self.
    fn count_ones(&self) -> usize;

    ///Returns the number of trailing zeros in the binary representation of self.
    fn trailing_zeros(&self) -> usize;

    ///Return the memory representation of this BitStorage as a byte array in big-endian (network) byte order.
    fn to_be_bytes(&self) -> Vec<u8>;

    /// Create a BitStorage value from its representation as a byte array in big endian.
    /// Returns a Result that contains either :
    /// - a valid BitStorage
    /// - a BitMaskError if the transformation failed (for example if the number of bytes given in parameter is not equal to SIZE/8)
    fn from_be_bytes(value: &[u8]) -> Result<Self, BitMaskError>
    where
        Self: Sized;
}

macro_rules! bit_storage_impl_primitive {
    ($t : ident) => {
        impl BitStorage for $t {
            const SIZE: usize = $t::BITS as usize;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn count_ones(&self) -> usize {
                $t::count_ones(*self) as usize
            }

            fn trailing_zeros(&self) -> usize {
                $t::trailing_zeros(*self) as usize
            }

            fn to_be_bytes(&self) -> Vec<u8> {
                $t::to_be_bytes(*self).to_vec()
            }

            fn from_be_bytes(value: &[u8]) -> Result<Self, BitMaskError> {
                Ok($t::from_be_bytes(
                    value
                        .try_into()
                        .map_err(|_| BitMaskError::DeserializationFailed)?,
                ))
            }
        }
    };
}

bit_storage_impl_primitive!(u8);
bit_storage_impl_primitive!(u16);
bit_storage_impl_primitive!(u32);
bit_storage_impl_primitive!(u64);
bit_storage_impl_primitive!(u128);
