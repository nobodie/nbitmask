#![deny(clippy::all)]

use std::fmt;

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

use std::{
    fmt::{Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

#[derive(Clone, Debug)]
pub struct BitMask {
    mask: Vec<u64>,
    size: u32,
}

impl BitMask {
    pub fn zeros(size: u32) -> Self {
        Self {
            mask: vec![0; (size / 64) as usize + 1],
            size,
        }
    }

    pub fn ones(size: u32) -> Self {
        let mut mask = Self::zeros(size);
        mask.set_all(true);
        mask
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn set_all(&mut self, value: bool) {
        let s = if value { !0x0 } else { 0x0 };
        for m in &mut self.mask {
            *m = s;
        }

        let last = self.size / 64;
        if let Some(m) = self.mask.get_mut(last as usize) {
            let offset = self.size % 64;
            let reset = (0x1u64 << offset) - 1;
            *m &= reset;
        }
    }

    pub fn set(&mut self, index: u32, value: bool) -> Result<(), Error> {
        let i = index / 64;
        let offset = index % 64;

        if let Some(m) = self.mask.get_mut(i as usize) {
            if value {
                *m |= 0x1 << offset;
            } else {
                *m &= !(0x1 << offset);
            }
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }

    pub fn get(&self, index: u32) -> Result<bool, Error> {
        let i = index / 64;
        let offset = index % 64;
        self.mask
            .get(i as usize)
            .map(|m| (m >> offset) & 0x1 == 0x1)
            .ok_or(Error::IndexOutOfBounds)
    }

    pub fn count_ones(&self) -> u32 {
        self.mask.iter().map(|m| m.count_ones()).sum()
    }

    pub fn trailing_zeros(&self) -> u32 {
        let mut acc = 0;
        for m in &self.mask {
            let t = m.trailing_zeros();
            if t != 64 {
                return acc + t;
            }
            acc += 64;
        }
        self.size
    }
}

impl PartialEq for BitMask {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
    }
}

impl Eq for BitMask {}

impl BitAnd for BitMask {
    type Output = BitMask;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = BitMask::zeros(self.size.max(rhs.size));

        for block_index in 0..res.mask.len() {
            res.mask[block_index] = (self.mask.get(block_index).map_or(0x0, |block| *block))
                & (rhs.mask.get(block_index).map_or(0x0, |block| *block));
        }

        res
    }
}

impl BitAndAssign for BitMask {
    fn bitand_assign(&mut self, rhs: Self) {
        for block_index in 0..self.mask.len() {
            self.mask[block_index] &= rhs.mask.get(block_index).map_or(0x0, |block| *block);
        }
    }
}

impl BitOr for BitMask {
    type Output = BitMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = BitMask::zeros(self.size.max(rhs.size));

        for block_index in 0..res.mask.len() {
            res.mask[block_index] = (self.mask.get(block_index).map_or(0x0, |block| *block))
                | (rhs.mask.get(block_index).map_or(0x0, |block| *block));
        }

        res
    }
}

impl BitOrAssign for BitMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask.resize(self.mask.len().max(rhs.mask.len()), 0);
        self.size = self.size.max(rhs.size);
        for block_index in 0..self.mask.len() {
            self.mask[block_index] |= rhs.mask.get(block_index).map_or(0x0, |block| *block);
        }
    }
}

impl Display for BitMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let mut rem = self.size as i32;
        for m in &self.mask {
            let size = rem.min(64); // min(rem, 64)
            s.push_str(
                &format!("{:#066b}", m)[(66 - size) as usize..]
                    .chars()
                    .rev()
                    .collect::<String>(),
            );
            rem -= 64;
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::BitMask;

    #[test]
    fn test_print() {
        let mask = BitMask::zeros(5);
        assert_eq!(mask.to_string(), "00000".to_string());

        let mask = BitMask::zeros(64);
        assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 64]).unwrap());

        let mask = BitMask::zeros(75);
        assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 75]).unwrap());

        let mut mask = BitMask::zeros(3);
        mask.set(1, true).unwrap();
        assert_eq!(mask.to_string(), "010".to_string());
    }

    #[test]
    fn test_get_set() {
        let mut mask = BitMask::zeros(5);
        mask.set(1, true).unwrap();
        assert_eq!(mask.get(0).unwrap(), false);
        assert_eq!(mask.get(1).unwrap(), true);
        assert_eq!(mask.to_string(), "01000".to_string());

        let mut mask = BitMask::zeros(5);
        mask.set_all(true);
        mask.set(1, false).unwrap();
        assert_eq!(mask.get(0).unwrap(), true);
        assert_eq!(mask.get(1).unwrap(), false);
        assert_eq!(mask.to_string(), "10111".to_string());
    }

    #[test]
    fn test_or() {
        let mut a = BitMask::zeros(3);
        let mut b = BitMask::zeros(3);

        a.set(1, true).unwrap();
        b.set(2, true).unwrap();

        assert_eq!((a | b).to_string(), "011".to_string());
    }

    #[test]
    fn test_or_assign() {
        let mut a = BitMask::ones(3);
        let mut b = BitMask::ones(4);
        a.set(1, false).unwrap();
        b.set(1, false).unwrap();

        a |= b;

        assert_eq!(a.to_string(), "1011".to_string());
    }

    #[test]
    fn test_and() {
        let mut a = BitMask::zeros(3);
        let mut b = BitMask::zeros(3);
        a.set_all(true);
        b.set_all(true);
        a.set(1, false).unwrap();
        b.set(2, false).unwrap();

        assert_eq!((a & b).to_string(), "100".to_string());
    }

    #[test]
    fn test_and_assign() {
        let mut a = BitMask::zeros(3);
        let mut b = BitMask::zeros(100);
        a.set_all(true);
        b.set_all(true);
        a.set(1, false).unwrap();
        b.set(2, false).unwrap();

        a &= b;

        assert_eq!(a.to_string(), "100".to_string());
    }

    #[test]
    fn test_eq() {
        let mut a = BitMask::zeros(3);
        let mut b = BitMask::zeros(3);

        a.set_all(true);

        a.set(1, false).unwrap();
        b.set(0, true).unwrap();
        b.set(2, true).unwrap();

        println!("{}, {}", a, b);

        assert_eq!(a, b);
    }

    #[test]
    fn test_ne() {
        let mut a = BitMask::zeros(3);
        let mut b = BitMask::zeros(3);

        a.set(1, true).unwrap();
        b.set(2, true).unwrap();

        assert_ne!(a, b);
    }
}
