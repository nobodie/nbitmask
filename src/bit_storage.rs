pub trait BitStorage {
    const SIZE: usize;
    const ZERO: Self;
    const ONE: Self;

    fn count_ones(&self) -> usize;
    fn trailing_zeros(&self) -> usize;
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
        }
    };
}

bit_storage_impl_primitive!(u8);
bit_storage_impl_primitive!(u16);
bit_storage_impl_primitive!(u32);
bit_storage_impl_primitive!(u64);
bit_storage_impl_primitive!(u128);
