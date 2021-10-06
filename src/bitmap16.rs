use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Sub, SubAssign,
    },
};

const MAP_LENGTH: u64 = 16;

/// The next size up bitmap of length 16.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// let mut bitmap = Bitmap16::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// // Will show 0000000000000000
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_u16());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= Bitmap64::from(101);
///
/// // Will show 0000000001100101
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 16
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_u64());
/// ```
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Bitmap16(u16);

impl Bitmap16 {
    pub fn to_u16(&self) -> u16 {
        self.0
    }

    /// Creates a new, empty `Bitmap16`, and sets the desired index before returning.
    ///
    /// This is equivalent to:
    /// ```rust
    /// let mut bitmap = Bitmap16::from(0);
    /// bitmap.set(index);
    /// ```
    pub fn from_set(index: u64) -> Option<Bitmap16> {
        if index >= MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap16::default();
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
            let mask = u16::MAX - (1 << index);
            self.0 &= mask;
        }

        Ok(())
    }

    pub fn get(&self, index: u64) -> Result<bool, String> {
        if index >= MAP_LENGTH {
            return Err(String::from(
                "Tried to get bit that's out of range of the bitmap (range: ",
            ) + &MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let mask = 1 << index;
        Ok(self.0 & mask > 0)
    }
}

impl From<u16> for Bitmap16 {
    fn from(value: u16) -> Self {
        Bitmap16(value)
    }
}

impl Display for Bitmap16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut bitmap = String::new();
        for i in 0..MAP_LENGTH {
            bitmap.push_str(&(if self.0 & (1 << i) > 0 { 1 } else { 0 }).to_string());
        }
        write!(f, "{}", bitmap.chars().rev().collect::<String>())
    }
}

// Traits implementing bitwise operations between Bitmaps of the same type

impl BitAnd for Bitmap16 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitmap16 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitmap16 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitmap16 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitmap16 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitmap16 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// Traits implementing arithmetic operations between Bitmaps of the same type

impl Add for Bitmap16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Bitmap16 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Bitmap16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Bitmap16 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Bitmap16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Bitmap16 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Bitmap16 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Bitmap16 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective integer types.

impl BitAnd<u16> for Bitmap16 {
    type Output = Self;

    fn bitand(self, rhs: u16) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<u16> for Bitmap16 {
    fn bitand_assign(&mut self, rhs: u16) {
        self.0 &= rhs;
    }
}

impl BitOr<u16> for Bitmap16 {
    type Output = Self;

    fn bitor(self, rhs: u16) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<u16> for Bitmap16 {
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs;
    }
}

impl BitXor<u16> for Bitmap16 {
    type Output = Self;

    fn bitxor(self, rhs: u16) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign<u16> for Bitmap16 {
    fn bitxor_assign(&mut self, rhs: u16) {
        self.0 ^= rhs;
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective integer types.

impl Add<u16> for Bitmap16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u16> for Bitmap16 {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs;
    }
}

impl Sub<u16> for Bitmap16 {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u16> for Bitmap16 {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 -= rhs;
    }
}

impl Mul<u16> for Bitmap16 {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<u16> for Bitmap16 {
    fn mul_assign(&mut self, rhs: u16) {
        self.0 *= rhs;
    }
}

impl Div<u16> for Bitmap16 {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<u16> for Bitmap16 {
    fn div_assign(&mut self, rhs: u16) {
        self.0 /= rhs;
    }
}