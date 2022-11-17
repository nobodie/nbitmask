#![deny(clippy::all)]

pub mod bit_storage;
pub mod error;

use std::fmt::Binary;

use std::ops::{Not, Shl, Shr, Sub};

use std::{
    fmt::{Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use bit_storage::BitStorage;
use error::BitMaskError;

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
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), BitMaskError> {
        if index >= self.size {
            return Err(BitMaskError::IndexOutOfBounds);
        }
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
            Err(BitMaskError::IndexOutOfBounds)
        }
    }
}

impl<T> BitMask<T>
where
    T: BitStorage + BitAnd<Output = T> + Clone + PartialEq + Shr<usize, Output = T>,
{
    pub fn get(&self, index: usize) -> Result<bool, BitMaskError> {
        if index >= self.size {
            return Err(BitMaskError::IndexOutOfBounds);
        }
        let i = index / T::SIZE;
        let offset = index % T::SIZE;
        self.mask
            .get(i)
            .map(|m| (m.clone() >> offset) & T::ONE == T::ONE)
            .ok_or(BitMaskError::IndexOutOfBounds)
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
            let size = rem.min(T::SIZE as isize) as usize; // min(rem, 64)

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
