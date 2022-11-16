#![deny(clippy::all)]

use std::fmt::{self, Binary};

#[derive(Clone, Debug)]
pub enum Error {
    IndexOutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IndexOutOfBounds => write!(f, "IndexOutOfBounds"),
        }
    }
}

use std::ops::{Not, Shl, Shr, Sub};

use std::{
    fmt::{Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

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

#[derive(Clone, Debug)]
pub struct BitMask<T> {
    mask: Vec<T>,
    size: usize,
}

impl<T> BitMask<T>
where
    T: BitStorage + Clone,
{
    pub fn zeros(size: usize) -> BitMask<T> {
        BitMask {
            mask: vec![T::ZERO; (size / T::SIZE) + 1],
            size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
    pub fn count_ones(&self) -> usize {
        self.mask.iter().map(|m| m.count_ones()).sum()
    }

    pub fn trailing_zeros(&self) -> usize {
        let mut acc = 0;
        for m in &self.mask {
            let t = m.trailing_zeros();
            if t != T::SIZE {
                return acc + t;
            }
            acc += T::SIZE;
        }
        self.size
    }
}

impl<T> BitMask<T>
where
    T: BitStorage
        + Not<Output = T>
        + Clone
        + BitAndAssign
        + Shl<usize, Output = T>
        + Sub<Output = T>,
{
    pub fn ones(size: usize) -> BitMask<T> {
        let mut mask = Self::zeros(size);
        mask.set_all(true);
        mask
    }

    pub fn set_all(&mut self, value: bool) {
        let s = if value { !T::ZERO } else { T::ZERO };
        for m in &mut self.mask {
            *m = s.clone();
        }

        let last = self.size / T::SIZE;
        if let Some(m) = self.mask.get_mut(last) {
            let offset = self.size % T::SIZE;
            *m &= (T::ONE << offset) - T::ONE;
        }
    }
}

impl<T> BitMask<T>
where
    T: BitStorage + Not<Output = T> + BitAndAssign + BitOrAssign + Shl<usize, Output = T>,
{
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), Error> {
        let i = index / T::SIZE;
        let offset = index % T::SIZE;

        if let Some(m) = self.mask.get_mut(i) {
            if value {
                *m |= T::ONE << offset;
            } else {
                *m &= !(T::ONE << offset);
            }
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }
}

impl<T> BitMask<T>
where
    T: BitStorage + BitAnd<Output = T> + Clone + PartialEq + Shr<usize, Output = T>,
{
    pub fn get(&self, index: usize) -> Result<bool, Error> {
        let i = index / T::SIZE;
        let offset = index % T::SIZE;
        self.mask
            .get(i)
            .map(|m| (m.clone() >> offset) & T::ONE == T::ONE)
            .ok_or(Error::IndexOutOfBounds)
    }
}

impl<T: PartialEq> PartialEq for BitMask<T> {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.size == other.size
    }
}

impl<T: PartialEq> Eq for BitMask<T> {}

impl<T> BitOr for BitMask<T>
where
    T: BitStorage
        + BitOr<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + BitAndAssign
        + BitOrAssign
        + PartialEq,
{
    type Output = BitMask<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = BitMask::zeros(self.size.max(rhs.size));

        for block_index in 0..res.mask.len() {
            res.mask[block_index] = (self
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone()))
                | (rhs
                    .mask
                    .get(block_index)
                    .map_or(T::ZERO, |block| block.clone()));
        }

        res
    }
}

impl<T> BitOrAssign for BitMask<T>
where
    T: BitStorage
        + BitOr<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + BitAndAssign
        + BitOrAssign
        + PartialEq,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask
            .resize(self.mask.len().max(rhs.mask.len()), T::ZERO);
        self.size = self.size.max(rhs.size);
        for block_index in 0..self.mask.len() {
            self.mask[block_index] |= rhs
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone());
        }
    }
}

impl<T> BitAnd for BitMask<T>
where
    T: BitStorage
        + BitOr<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + BitAndAssign
        + BitOrAssign
        + PartialEq,
{
    type Output = BitMask<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = BitMask::zeros(self.size.max(rhs.size));

        for block_index in 0..res.mask.len() {
            res.mask[block_index] = (self
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone()))
                & (rhs
                    .mask
                    .get(block_index)
                    .map_or(T::ZERO, |block| block.clone()));
        }

        res
    }
}

impl<T> BitAndAssign for BitMask<T>
where
    T: BitStorage
        + BitOr<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + Not<Output = T>
        + BitAnd<Output = T>
        + Clone
        + BitAndAssign
        + BitOrAssign
        + PartialEq,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for block_index in 0..self.mask.len() {
            self.mask[block_index] &= rhs
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone());
        }
    }
}

impl<T: BitStorage + Display + Binary> Display for BitMask<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let mut rem = self.size as isize;
        for m in &self.mask {
            dbg!(rem);

            let size = rem.min(T::SIZE as isize) as usize; // min(rem, 64)
            dbg!(size);

            s.push_str(
                &format!("{:#0w$b}", m, w = T::SIZE + 2)[(T::SIZE + 2 - size)..]
                    .chars()
                    .rev()
                    .collect::<String>(),
            );
            rem -= T::SIZE as isize;
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::BitMask;

    #[test]
    fn test_print() {
        let mask: BitMask<u64> = BitMask::zeros(5);
        assert_eq!(mask.to_string(), "00000".to_string());

        let mask: BitMask<u64> = BitMask::zeros(64);
        assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 64]).unwrap());

        let mask: BitMask<u64> = BitMask::zeros(75);
        assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 75]).unwrap());

        let mut mask: BitMask<u64> = BitMask::zeros(3);
        mask.set(1, true).unwrap();
        assert_eq!(mask.to_string(), "010".to_string());
    }

    #[test]
    fn test_get_set_u64() {
        let mut mask: BitMask<u64> = BitMask::zeros(5);
        mask.set(1, true).unwrap();
        assert_eq!(mask.get(0).unwrap(), false);
        assert_eq!(mask.get(1).unwrap(), true);
        assert_eq!(mask.to_string(), "01000".to_string());

        let mut mask: BitMask<u64> = BitMask::zeros(5);
        mask.set_all(true);
        mask.set(1, false).unwrap();
        assert_eq!(mask.get(0).unwrap(), true);
        assert_eq!(mask.get(1).unwrap(), false);
        assert_eq!(mask.to_string(), "10111".to_string());
    }

    #[test]
    fn test_get_set_u16() {
        let mut mask: BitMask<u16> = BitMask::zeros(17);
        mask.set(1, true).unwrap();
        assert_eq!(mask.get(0).unwrap(), false);
        assert_eq!(mask.get(1).unwrap(), true);
        assert_eq!(mask.to_string(), "01000000000000000".to_string());

        let mut mask: BitMask<u16> = BitMask::zeros(17);
        mask.set_all(true);
        mask.set(1, false).unwrap();
        assert_eq!(mask.get(0).unwrap(), true);
        assert_eq!(mask.get(1).unwrap(), false);
        assert_eq!(mask.to_string(), "10111111111111111".to_string());
    }

    #[test]
    fn test_get_set_u8() {
        let mut mask: BitMask<u8> = BitMask::zeros(257);
        mask.set(1, true).unwrap();
        assert_eq!(mask.get(0).unwrap(), false);
        assert_eq!(mask.get(1).unwrap(), true);
        assert_eq!(mask.to_string(), "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string());

        let mut mask: BitMask<u8> = BitMask::zeros(257);
        mask.set_all(true);
        mask.set(1, false).unwrap();
        assert_eq!(mask.get(0).unwrap(), true);
        assert_eq!(mask.get(1).unwrap(), false);
        assert_eq!(mask.to_string(), "10111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string());
    }

    #[test]
    fn test_equals() {
        let mut mask1: BitMask<u8> = BitMask::zeros(5);
        mask1.set(1, true).unwrap();

        let mut mask2: BitMask<u8> = BitMask::zeros(5);
        mask2.set(1, true).unwrap();

        assert_eq!(mask1, mask2);
    }

    #[test]
    fn test_not_equals() {
        let mut mask1: BitMask<u8> = BitMask::zeros(6);
        mask1.set(1, true).unwrap();

        let mut mask2: BitMask<u8> = BitMask::zeros(5);
        mask2.set(1, true).unwrap();

        assert_ne!(mask1, mask2);
    }

    #[test]
    fn test_or() {
        let mut a: BitMask<u64> = BitMask::zeros(3);
        let mut b: BitMask<u64> = BitMask::zeros(3);

        a.set(1, true).unwrap();
        b.set(2, true).unwrap();

        assert_eq!((a | b).to_string(), "011".to_string());
    }

    #[test]
    fn test_or_assign() {
        let mut a: BitMask<u64> = BitMask::ones(3);
        let mut b: BitMask<u64> = BitMask::ones(4);
        a.set(1, false).unwrap();
        b.set(1, false).unwrap();

        a |= b;

        assert_eq!(a.to_string(), "1011".to_string());
    }

    #[test]
    fn test_and() {
        let mut a: BitMask<u64> = BitMask::zeros(3);
        let mut b: BitMask<u64> = BitMask::zeros(3);
        a.set_all(true);
        b.set_all(true);
        a.set(1, false).unwrap();
        b.set(2, false).unwrap();

        assert_eq!((a & b).to_string(), "100".to_string());
    }

    #[test]
    fn test_and_assign() {
        let mut a: BitMask<u64> = BitMask::zeros(3);
        let mut b: BitMask<u64> = BitMask::zeros(100);
        a.set_all(true);
        b.set_all(true);
        a.set(1, false).unwrap();
        b.set(2, false).unwrap();

        a &= b;

        assert_eq!(a.to_string(), "100".to_string());
    }
}
