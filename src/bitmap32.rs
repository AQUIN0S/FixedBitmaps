use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

const MAP_LENGTH: u64 = 32;

/// A bitmap of length 32. This would be the fastest bitmap to use on 32-bit architectures.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// let mut bitmap = Bitmap32::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// // Will show 00000000000000000000000000000000
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_u32());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= Bitmap64::from(101);
///
/// // Will show 00000000000000000000000001100101
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 32
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_u64());
/// ```
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Bitmap32(u32);

impl Bitmap32 {
    pub fn to_u32(&self) -> u32 {
        self.0
    }

    /// Creates a new, empty `Bitmap32`, and sets the desired index before returning.
    ///
    /// This is equivalent to:
    /// ```rust
    /// let mut bitmap = Bitmap32::from(0);
    /// bitmap.set(index);
    /// ```
    pub fn from_set(index: u64) -> Option<Bitmap32> {
        if index >= MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap32::default();
        bitmap.set(index, true).unwrap();
        Some(bitmap)
    }

    pub fn set(&mut self, index: u64, value: bool) -> Result<(), String> {
        if index >= MAP_LENGTH {
            return Err(String::from(
                "Tried to set bit that's out of range of the bitmap (range: ",
            ) + &MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        if value {
            let mask = 1 << index;
            self.0 |= mask;
        } else {
            let mask = u32::MAX - (1 << index);
            self.0 &= mask;
        }

        Ok(())
    }

    pub fn get(&self, index: u64) -> Result<bool, String> {
        if index >= MAP_LENGTH {
            return Err(String::from(
                "Tried to set bit that's out of range of the bitmap (range: ",
            ) + &MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let mask = 1 << index;
        Ok(self.0 & mask > 0)
    }
}

impl From<u32> for Bitmap32 {
    fn from(value: u32) -> Self {
        Bitmap32(value)
    }
}

impl Display for Bitmap32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut bitmap = String::new();
        for i in 0..MAP_LENGTH {
            bitmap.push_str(&(if self.0 & (1 << i) > 0 { 1 } else { 0 }).to_string());
        }
        write!(f, "{}", bitmap.chars().rev().collect::<String>())
    }
}

impl BitAnd for Bitmap32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitmap32 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitmap32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitmap32 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitmap32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitmap32 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
