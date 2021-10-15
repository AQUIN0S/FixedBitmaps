use super::ConstantLength;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    mem,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Div,
        DivAssign, Mul, MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

/// A bitmap of length 8.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// use fixed_bitmaps::Bitmap8;
///
/// let mut bitmap = Bitmap8::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_u8());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= Bitmap8::from(101);
///
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 8
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_u8());
///
/// // Or you could use the deref operator for an even easier conversion
/// println!("Bitmap value: {}", *bitmap);
/// ```
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Bitmap8(u8);

impl Bitmap8 {
    pub fn capacity() -> usize {
        Bitmap8::MAP_LENGTH
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn new(value: bool) -> Bitmap8 {
        Bitmap8(if value { u8::MAX } else { 0 })
    }

    /// Create a new bitmap that has its bits set from `begin` (inclusive) to `end` (exclusive).
    /// If begin is greater than the map length or end is 0, will return an empty bitmap.
    pub fn create_bit_mask(begin: usize, end: usize, value: bool) -> Bitmap8 {
        if value {
            if begin >= Bitmap8::MAP_LENGTH || end < 1 {
                Bitmap8(0)
            } else if end >= Bitmap8::MAP_LENGTH {
                Bitmap8(u8::MAX << begin)
            } else {
                Bitmap8(u8::MAX << begin & u8::MAX >> Bitmap8::MAP_LENGTH - end)
            }
        } else {
            if begin >= Bitmap8::MAP_LENGTH || end < 1 {
                Bitmap8(u8::MAX)
            } else if begin == 0 && end >= Bitmap8::MAP_LENGTH {
                Bitmap8(0)
            } else if end >= Bitmap8::MAP_LENGTH {
                Bitmap8(u8::MAX >> Bitmap8::MAP_LENGTH - begin)
            } else if begin < 1 {
                Bitmap8(u8::MAX << end)
            } else {
                Bitmap8(u8::MAX >> Bitmap8::MAP_LENGTH - begin | u8::MAX << end)
            }
        }
    }

    /// Creates a new, empty `Bitmap8`, and sets the desired index before returning.
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap8;
    ///
    /// let a = Bitmap8::from_set(5).unwrap();
    ///
    /// // The above is equivalent to:
    ///
    /// let mut b = Bitmap8::from(0);
    /// b.set(5, true);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn from_set(index: usize) -> Option<Bitmap8> {
        if index >= Bitmap8::MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap8::default();
        bitmap.set(index, true).unwrap();
        Some(bitmap)
    }

    /// Sets the desired index, to the value provided. Note that indexing starts
    /// at 0.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` based on the outcome. If an `Err<String>` was returned,
    /// it was because an out-of-bounds index was attempted to be set. In that
    /// case the bitmap's state remains unchanged.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap8;
    ///
    /// let mut bitmap = Bitmap8::default();
    /// assert_eq!(*bitmap, 0);
    ///
    /// bitmap.set(4, true);
    /// assert_eq!(*bitmap, 16);
    /// ```
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), String> {
        if index >= Bitmap8::MAP_LENGTH {
            return Err(String::from(
                "Tried to set bit that's out of range of the bitmap (range: ",
            ) + &Bitmap8::MAP_LENGTH.to_string()
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

    pub fn set_range(&mut self, begin: usize, end: usize, value: bool) {
        if value {
            *self |= Bitmap8::create_bit_mask(begin, end, true);
        } else {
            *self &= Bitmap8::create_bit_mask(begin, end, false);
        }
    }

    /// Gets the bit at the given index. Note that indexing starts at 0.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` based on the outcome.
    ///
    /// If `Ok<bool>` is returned, then the contained value in ok is the state
    /// of the given bit
    ///
    /// If an `Err<String>` was returned, it was because you tried to get
    /// an out-of-bounds index.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap8;
    ///
    /// let bitmap = Bitmap8::from(0b1010);
    /// assert_eq!(bitmap.get(2).unwrap(), false);
    /// assert_eq!(bitmap.get(3).unwrap(), true);
    /// ```
    pub fn get(&self, index: usize) -> Result<bool, String> {
        if index >= Bitmap8::MAP_LENGTH {
            return Err(String::from(
                "Tried to get bit that's out of range of the bitmap (range: ",
            ) + &Bitmap8::MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let mask = 1 << index;
        Ok(self.0 & mask > 0)
    }
}

impl Display for Bitmap8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:b}", self.0)
    }
}

impl From<u8> for Bitmap8 {
    fn from(value: u8) -> Self {
        Bitmap8(value)
    }
}

impl ConstantLength for Bitmap8 {
    const MAP_LENGTH: usize = mem::size_of::<u8>() * 8;
}

// Traits implementing bitwise operations between Bitmaps of the same type

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

// Traits implementing arithmetic operations between Bitmaps of the same type

impl Add for Bitmap8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Bitmap8 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Bitmap8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Bitmap8 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Bitmap8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Bitmap8 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Bitmap8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Bitmap8 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective integer types.

impl BitAnd<u8> for Bitmap8 {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<u8> for Bitmap8 {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= rhs;
    }
}

impl BitOr<u8> for Bitmap8 {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOrAssign<u8> for Bitmap8 {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}

impl BitXor<u8> for Bitmap8 {
    type Output = Self;

    fn bitxor(self, rhs: u8) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign<u8> for Bitmap8 {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= rhs;
    }
}

// Traits implementing arithmetic operations between Bitmaps and their respective integer types.

impl Add<u8> for Bitmap8 {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u8> for Bitmap8 {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;
    }
}

impl Sub<u8> for Bitmap8 {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u8> for Bitmap8 {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;
    }
}

impl Mul<u8> for Bitmap8 {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<u8> for Bitmap8 {
    fn mul_assign(&mut self, rhs: u8) {
        self.0 *= rhs;
    }
}

impl Div<u8> for Bitmap8 {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<u8> for Bitmap8 {
    fn div_assign(&mut self, rhs: u8) {
        self.0 /= rhs;
    }
}

// Traits for left and right bitwise shifts. These really only make sense when working
// with integers, rather than other bitmaps

impl Shl<usize> for Bitmap8 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl ShlAssign<usize> for Bitmap8 {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl Shr<usize> for Bitmap8 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShrAssign<usize> for Bitmap8 {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

// The Not trait, flipping 1's to 0's and 0's to 1's

impl Not for Bitmap8 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0 ^ u8::MAX)
    }
}

// Dereference

impl Deref for Bitmap8 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
