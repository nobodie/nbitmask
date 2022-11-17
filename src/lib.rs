#![deny(clippy::all)]

pub mod bit_storage;
pub mod error;

use std::fmt::Binary;

use std::ops::{BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub};

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

impl<T> BitOrAssign<&Self> for BitMask<T>
where
    T: BitStorage + Clone + BitOrAssign,
{
    fn bitor_assign(&mut self, rhs: &Self) {
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

impl<'a, T: 'a> BitOr<Self> for &'a BitMask<T>
where
    BitMask<T>: BitOrAssign<&'a BitMask<T>> + Clone,
{
    type Output = BitMask<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res |= rhs;
        res
    }
}

impl<T> BitXorAssign<&Self> for BitMask<T>
where
    T: BitStorage + Clone + BitXorAssign,
{
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.mask
            .resize(self.mask.len().max(rhs.mask.len()), T::ZERO);
        self.size = self.size.max(rhs.size);
        for block_index in 0..self.mask.len() {
            self.mask[block_index] ^= rhs
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone());
        }
    }
}

impl<'a, T: 'a> BitXor<Self> for &'a BitMask<T>
where
    BitMask<T>: BitXorAssign<&'a BitMask<T>> + Clone,
{
    type Output = BitMask<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res ^= rhs;
        res
    }
}

impl<T> BitAndAssign<&Self> for BitMask<T>
where
    T: BitStorage + Clone + BitAndAssign,
{
    fn bitand_assign(&mut self, rhs: &Self) {
        for block_index in 0..self.mask.len() {
            self.mask[block_index] &= rhs
                .mask
                .get(block_index)
                .map_or(T::ZERO, |block| block.clone());
        }
    }
}

impl<'a, T: 'a> BitAnd<Self> for &'a BitMask<T>
where
    BitMask<T>: BitAndAssign<&'a BitMask<T>> + Clone,
{
    type Output = BitMask<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res &= rhs;
        res
    }
}

impl<T> Not for &BitMask<T>
where
    T: BitStorage
        + Not<Output = T>
        + Clone
        + BitAndAssign
        + Shl<usize, Output = T>
        + Sub<Output = T>,
{
    type Output = BitMask<T>;

    fn not(self) -> Self::Output {
        let mut res = self.clone();

        for m in res.mask.iter_mut() {
            *m = !m.clone();
        }

        //Performing a bitwise and on the last BitStorage unit to remove the remaining bits eventually set to 1 by not operator
        let mask: BitMask<T> = BitMask::ones(res.size % T::SIZE);
        res.mask[res.size / T::SIZE] &= mask.mask[0].clone();

        res
    }
}

impl<T> ShrAssign<usize> for BitMask<T>
where
    T: BitStorage + ShrAssign<usize> + ShlAssign<usize> + BitOrAssign + Clone + std::fmt::Debug,
{
    fn shr_assign(&mut self, rhs: usize) {
        for index in 0..self.mask.len() {
            let index_block_to_get_data_from = index + (rhs / T::SIZE);

            let mut block_copy_to_get_data_from = self
                .mask
                .get(index_block_to_get_data_from)
                .unwrap_or(&T::ZERO)
                .clone();

            let offset_into_block_to_get_data_from = rhs % T::SIZE;

            block_copy_to_get_data_from >>= offset_into_block_to_get_data_from;

            let mut next_block_copy_to_get_data_from = self
                .mask
                .get(index_block_to_get_data_from + 1)
                .unwrap_or(&T::ZERO)
                .clone();

            next_block_copy_to_get_data_from <<= T::SIZE - (rhs % T::SIZE);
            block_copy_to_get_data_from |= next_block_copy_to_get_data_from;

            self.mask[index] = block_copy_to_get_data_from;

            /*if rhs >= T::SIZE {
                            self.mask[index] = T::ZERO;
                        } else {
                            self.mask[index] >>= rhs;
                        }

                        println!("index {}", index);

                        let block_to_shift_index = index + (rhs / T::SIZE);
                        println!("index block to shift from {}", block_to_shift_index);

                        let mut block_to_shift_val = self
                            .mask
                            .get(block_to_shift_index)
                            .unwrap_or(&T::ZERO)
                            .clone();

                        println!("block value : {:?}", block_to_shift_val);

                        println!("shift value : {:?}", T::SIZE - (rhs % T::SIZE));

                        block_to_shift_val <<= T::SIZE - (rhs % T::SIZE);
                        println!("block value : {:?}", block_to_shift_val);

            self.mask[index] |= block_to_shift_val;*/
        }
    }
}

impl<T> Shr<usize> for &BitMask<T>
where
    BitMask<T>: ShrAssign<usize> + Clone,
{
    type Output = BitMask<T>;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut res = self.clone();
        (res >>= rhs);
        res
    }
}

impl<T> ShlAssign<usize> for BitMask<T>
where
    T: BitStorage + ShlAssign<usize> + ShrAssign<usize> + BitOrAssign + Clone + std::fmt::Debug,
{
    fn shl_assign(&mut self, rhs: usize) {
        for index in (0..self.mask.len()).rev() {
            if rhs >= T::SIZE {
                self.mask[index] = T::ZERO;
            } else {
                self.mask[index] <<= rhs;
            }

            let block_to_shift_index = index as isize - (rhs / T::SIZE) as isize;

            let mut block_to_shift_val = self
                .mask
                .get(block_to_shift_index as usize)
                .unwrap_or(&T::ZERO)
                .clone();
            block_to_shift_val >>= T::SIZE - (rhs % T::SIZE);

            self.mask[index] |= block_to_shift_val;
        }
    }
}

impl<T> Shl<usize> for &BitMask<T>
where
    BitMask<T>: ShlAssign<usize> + Clone,
{
    type Output = BitMask<T>;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut res = self.clone();
        (res <<= rhs);
        res
    }
}

impl<T: BitStorage + Display + Binary> Display for BitMask<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let mut rem = self.size as isize;
        for m in &self.mask {
            let size = rem.min(T::SIZE as isize) as usize;

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
