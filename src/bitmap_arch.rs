use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    mem,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Sub, SubAssign,
    },
};

const MAP_LENGTH: u64 = (mem::size_of::<usize>() as u64) * 8;

/// A bitmap whose size depends on the architecture of the target computer it's running on. For example on a 64-bit target, this bitmap would be 8 bytes long,
/// whereas on a 32-bit target, it would hold 4 bytes of data.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// let mut bitmap = BitmapArch::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// // Will show 32 0's on a 32-bit architecture, and 64 on a 64-bit architecture
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_usize());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= BitmapArch::from(101);
///
/// // Will show ...1100101, where ... represents some number of padding 0's that depends on the architecture of the target
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 32
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_usize());
/// ```
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct BitmapArch(usize);

impl BitmapArch {
    pub fn to_usize(&self) -> usize {
        self.0
    }

    /// Creates a new, empty `Bitmap64`, and sets the desired index before returning.
    ///
    /// When calling:
    ///
    /// ```rust
    /// use fixed_bitmaps::BitmapArch;
    ///
    /// let mut bitmap = BitmapArch::from_set(5);
    /// ```
    ///
    /// This is equivalent to:
    ///
    /// ```rust
    /// use fixed_bitmaps::BitmapArch;
    ///
    /// let mut bitmap = BitmapArch::from(0);
    /// bitmap.set(5, true);
    /// ```
    pub fn from_set(index: u64) -> Option<BitmapArch> {
        if index >= MAP_LENGTH {
            return None;
        }

        let mut bitmap = BitmapArch::default();
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
            let mask = usize::MAX - (1 << index);
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

    pub fn capacity() -> u64 {
        MAP_LENGTH
    }
}

impl Display for BitmapArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut bitmap = String::new();
        for i in 0..MAP_LENGTH {
            bitmap.push_str(&(if self.0 & (1 << i) > 0 { 1 } else { 0 }).to_string());
        }
        write!(f, "{}", bitmap.chars().rev().collect::<String>())
    }
}

impl From<usize> for BitmapArch {
    fn from(value: usize) -> Self {
        BitmapArch(value)
    }
}

// Traits implementing bitwise operations between Bitmaps of the same type

impl BitAnd for BitmapArch {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitmapArch {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitmapArch {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitmapArch {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitmapArch {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitmapArch {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// Traits implementing arithmetic operations between Bitmaps of the same type

impl Add for BitmapArch {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for BitmapArch {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for BitmapArch {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for BitmapArch {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for BitmapArch {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for BitmapArch {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for BitmapArch {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for BitmapArch {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective integer types.

impl BitAnd<usize> for BitmapArch {
    type Output = Self;

    fn bitand(self, rhs: usize) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<usize> for BitmapArch {
    fn bitand_assign(&mut self, rhs: usize) {
        self.0 &= rhs;
    }
}

impl BitOr<usize> for BitmapArch {
    type Output = Self;

    fn bitor(self, rhs: usize) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<usize> for BitmapArch {
    fn bitor_assign(&mut self, rhs: usize) {
        self.0 |= rhs;
    }
}

impl BitXor<usize> for BitmapArch {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign<usize> for BitmapArch {
    fn bitxor_assign(&mut self, rhs: usize) {
        self.0 ^= rhs;
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective integer types.

impl Add<usize> for BitmapArch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for BitmapArch {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for BitmapArch {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for BitmapArch {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Mul<usize> for BitmapArch {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<usize> for BitmapArch {
    fn mul_assign(&mut self, rhs: usize) {
        self.0 *= rhs;
    }
}

impl Div<usize> for BitmapArch {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<usize> for BitmapArch {
    fn div_assign(&mut self, rhs: usize) {
        self.0 /= rhs;
    }
}
