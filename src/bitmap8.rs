use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

const MAP_LENGTH: u64 = 8;

/// The smallest denomination of bitmap in this crate. This is simply a byte-long bitmap,
/// useful for when only a few flags would be defined.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// let mut bitmap = Bitmap8::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// // Will show 00000000
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_u8());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= Bitmap64::from(101);
///
/// // Will show 01100101
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 8
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_u64());
/// ```
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Bitmap8(u8);

impl Bitmap8 {
    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn from_set(index: u64) -> Option<Bitmap8> {
        if index >= MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap8::default();
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
            let mask = u8::MAX - (1 << index);
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

impl From<u8> for Bitmap8 {
    fn from(value: u8) -> Self {
        Bitmap8(value)
    }
}

impl Display for Bitmap8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut bitmap = String::new();
        for i in 0..MAP_LENGTH {
            bitmap.push_str(&(if self.0 & (1 << i) > 0 { 1 } else { 0 }).to_string());
        }
        write!(f, "{}", bitmap.chars().rev().collect::<String>())
    }
}

impl BitAnd for Bitmap8 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitmap8 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitmap8 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitmap8 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitmap8 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitmap8 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
