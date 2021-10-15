use super::BitmapSize;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    mem,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Div,
        DivAssign, Mul, MulAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

/// A bitmap of length 16.
///
/// # Examples
/// ```rust
/// // Creates an empty bitmap
/// use fixed_bitmaps::Bitmap16;
///
/// let mut bitmap = Bitmap16::default();
///
/// // Bitmaps implement Display so you can view what the map looks like
/// println!("Default bitmap: {}", bitmap);
///
/// // Bitmaps also convert to their respective unsigned int versions and back again easily
/// // Will show 0 as the value of the bitmap
/// println!("Value of bitmap: {}", bitmap.to_u16());
///
/// // Let's do the same as above, but actually setting the values in the bitmap to something
/// bitmap |= Bitmap16::from(101);
///
/// println!("Bitmap after OR-ing with 101: {}", bitmap);
///
/// // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to ignore the warning,
/// //as we know for certain that 4 < 16
/// bitmap.set(4, true).unwrap();
///
/// // Will show that 117 (101 + 2^4) is the value of the bitmap
/// println!("Bitmap value: {}", bitmap.to_u16());
///
/// // Or you could use the deref operator for an even easier conversion
/// println!("Bitmap value: {}", *bitmap);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default, Serialize, Deserialize)]
pub struct Bitmap16(u16);

impl Bitmap16 {
    pub fn capacity() -> usize {
        Bitmap16::MAP_LENGTH
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }

    /// Creates a new bitmap with all bits set to the given value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let a = Bitmap16::new(true);
    /// assert_eq!(*a, u16::MAX);
    ///
    /// let b = Bitmap16::new(false);
    /// assert_eq!(*b, 0);
    /// ```
    pub fn new(value: bool) -> Bitmap16 {
        Bitmap16(if value { u16::MAX } else { 0 })
    }

    /// Create a new bitmap that has its bits set from `begin` (inclusive) to `end` (exclusive).
    /// If begin is greater than the map length or end is 0, will return a bitmap with all bits set to
    /// the opposite of value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let a = Bitmap16::create_bit_mask(3, 7, true);
    /// assert_eq!(*a, 0b1111000);
    ///
    /// let b = Bitmap16::create_bit_mask(3, 6, false); // Results in 1..1000111
    /// assert_eq!(b, Bitmap16::new(true) ^ 0b111000);
    /// ```
    pub fn create_bit_mask(begin: usize, end: usize, value: bool) -> Bitmap16 {
        if value {
            if begin >= Bitmap16::MAP_LENGTH || end < 1 {
                Bitmap16(0)
            } else if end >= Bitmap16::MAP_LENGTH {
                Bitmap16(u16::MAX << begin)
            } else {
                Bitmap16(u16::MAX << begin & u16::MAX >> Bitmap16::MAP_LENGTH - end)
            }
        } else {
            !Bitmap16::create_bit_mask(begin, end, true)
        }
    }

    /// Creates a new, empty `Bitmap16`, and sets the desired index before returning.
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let a = Bitmap16::from_set(5).unwrap();
    ///
    /// // The above is equivalent to:
    ///
    /// let mut b = Bitmap16::from(0);
    /// b.set(5, true);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn from_set(index: usize) -> Option<Bitmap16> {
        if index >= Bitmap16::MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap16::default();
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
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let mut bitmap = Bitmap16::default();
    /// assert_eq!(*bitmap, 0);
    ///
    /// bitmap.set(4, true);
    /// assert_eq!(*bitmap, 16);
    /// ```
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), String> {
        if index >= Bitmap16::MAP_LENGTH {
            return Err(String::from(
                "Tried to set bit that's out of range of the bitmap (range: ",
            ) + &Bitmap16::MAP_LENGTH.to_string()
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

    /// Set bits from begin (inclusive) to end (exclusive) to the given value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let mut bitmap = Bitmap16::default();
    /// assert_eq!(*bitmap, 0);
    ///
    /// bitmap.set_range(2, 7, true);
    /// assert_eq!(*bitmap, 0b1111100);
    ///
    /// bitmap.set_range(3, 5, false);
    /// assert_eq!(*bitmap, 0b1100100);
    /// ```
    pub fn set_range(&mut self, begin: usize, end: usize, value: bool) {
        if value {
            *self |= Bitmap16::create_bit_mask(begin, end, true);
        } else {
            *self &= Bitmap16::create_bit_mask(begin, end, false);
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
    /// use fixed_bitmaps::Bitmap16;
    ///
    /// let bitmap = Bitmap16::from(0b1010);
    /// assert_eq!(bitmap.get(2).unwrap(), false);
    /// assert_eq!(bitmap.get(3).unwrap(), true);
    /// ```
    pub fn get(&self, index: usize) -> Result<bool, String> {
        if index >= Bitmap16::MAP_LENGTH {
            return Err(String::from(
                "Tried to get bit that's out of range of the bitmap (range: ",
            ) + &Bitmap16::MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let mask = 1 << index;
        Ok(self.0 & mask > 0)
    }
}

impl Display for Bitmap16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:b}", self.0)
    }
}

impl Debug for Bitmap16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitmap16({:X})", self.0)
    }
}

impl From<u16> for Bitmap16 {
    fn from(value: u16) -> Self {
        Bitmap16(value)
    }
}

impl BitmapSize for Bitmap16 {
    const MAP_LENGTH: usize = mem::size_of::<u16>() * 8;
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

// Traits implementing arithmetic operations between Bitmaps and their respective integer types.

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

// Traits for left and right bitwise shifts. These really only make sense when working
// with integers, rather than other bitmaps

impl Shl<usize> for Bitmap16 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl ShlAssign<usize> for Bitmap16 {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl Shr<usize> for Bitmap16 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShrAssign<usize> for Bitmap16 {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

// The Not trait, flipping 1's to 0's and 0's to 1's

impl Not for Bitmap16 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0 ^ u16::MAX)
    }
}

// Dereference

impl Deref for Bitmap16 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
