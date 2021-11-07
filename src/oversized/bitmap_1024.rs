use core::fmt::Formatter;
use std::{
    fmt::Display,
    mem,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref},
};

use crate::BitmapSize;

const ELEMENT_SIZE: usize = mem::size_of::<usize>() * 8;
const ELEMENT_COUNT: usize = Bitmap1024::MAP_LENGTH / ELEMENT_SIZE;

/// Experimental struct for now, a bitmap containing 1_024 bits.
/// I wouldn't yet recommend using this struct until it's more stable!
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Bitmap1024([usize; ELEMENT_COUNT]);

impl Default for Bitmap1024 {
    fn default() -> Self {
        Self([0; ELEMENT_COUNT])
    }
}

impl Bitmap1024 {
    fn get_element_location(bit_index: usize) -> usize {
        ELEMENT_COUNT - 1 - bit_index / ELEMENT_SIZE
    }

    pub fn capacity() -> usize {
        Bitmap1024::MAP_LENGTH
    }

    pub fn to_array(&self) -> [usize; ELEMENT_COUNT] {
        self.0
    }

    pub fn get(&self, index: usize) -> Result<bool, String> {
        if index >= Bitmap1024::MAP_LENGTH {
            return Err(String::from(
                "Tried to get bit that's out of range of the bitmap (range: ",
            ) + &Bitmap1024::MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let element_location = Bitmap1024::get_element_location(index);
        let mask = 1 << index % ELEMENT_SIZE;
        Ok(self.0[element_location] & mask > 0)
    }

    pub fn set(&mut self, index: usize, value: bool) -> Result<(), String> {
        if index >= Bitmap1024::MAP_LENGTH {
            return Err(String::from(
                "Tried to set bit that's out of range of the bitmap (range: ",
            ) + &Bitmap1024::MAP_LENGTH.to_string()
                + ", index: "
                + &index.to_string()
                + ")");
        }

        let element_location = Bitmap1024::get_element_location(index);

        if value {
            let mask = 1 << index % ELEMENT_SIZE;
            self.0[element_location] |= mask;
        } else {
            let mask = usize::MAX - (1 << index % ELEMENT_SIZE);
            self.0[element_location] &= mask;
        }

        Ok(())
    }

    pub fn from_set(index: usize) -> Option<Bitmap1024> {
        if index >= Bitmap1024::MAP_LENGTH {
            return None;
        }

        let mut bitmap = Bitmap1024::default();
        bitmap.set(index, true).unwrap();
        Some(bitmap)
    }

    pub fn new(value: bool) -> Bitmap1024 {
        Bitmap1024(if value {
            [usize::MAX; ELEMENT_COUNT]
        } else {
            [0; ELEMENT_COUNT]
        })
    }
}

impl Display for Bitmap1024 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut bitmap = String::new();
        for i in 0..ELEMENT_COUNT {
            bitmap.push_str(format!("{:X}", self.0[i]).as_str());
            if i < ELEMENT_COUNT - 1 {
                bitmap.push_str("_");
            }
        }
        write!(f, "{}", bitmap.chars().collect::<String>())
    }
}

impl BitmapSize for Bitmap1024 {
    const MAP_LENGTH: usize = 1_024;
}

impl From<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    fn from(value: [usize; ELEMENT_COUNT]) -> Self {
        Bitmap1024(value)
    }
}

// Traits implementing bitwise operations between Bitmaps of the same type

impl BitAnd for Bitmap1024 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] &= rhs.0[i];
        }
        Self(bitmap)
    }
}

impl BitAndAssign for Bitmap1024 {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] &= rhs.0[i];
        }
    }
}

impl BitOr for Bitmap1024 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] |= rhs.0[i];
        }
        Self(bitmap)
    }
}

impl BitOrAssign for Bitmap1024 {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] |= rhs.0[i];
        }
    }
}

impl BitXor for Bitmap1024 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] ^= rhs.0[i];
        }
        Self(bitmap)
    }
}

impl BitXorAssign for Bitmap1024 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] ^= rhs.0[i];
        }
    }
}

// Traits implementing bitwise operations between Bitmaps and their respective array type

impl BitAnd<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    type Output = Self;

    fn bitand(self, rhs: [usize; ELEMENT_COUNT]) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] &= rhs[i];
        }
        Self(bitmap)
    }
}

impl BitAndAssign<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    fn bitand_assign(&mut self, rhs: [usize; ELEMENT_COUNT]) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] &= rhs[i];
        }
    }
}

impl BitOr<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    type Output = Self;

    fn bitor(self, rhs: [usize; ELEMENT_COUNT]) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] |= rhs[i];
        }
        Self(bitmap)
    }
}

impl BitOrAssign<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    fn bitor_assign(&mut self, rhs: [usize; ELEMENT_COUNT]) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] |= rhs[i];
        }
    }
}

impl BitXor<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    type Output = Self;

    fn bitxor(self, rhs: [usize; ELEMENT_COUNT]) -> Self::Output {
        let mut bitmap = self.0;
        for i in 0..ELEMENT_COUNT {
            bitmap[i] ^= rhs[i];
        }
        Self(bitmap)
    }
}

impl BitXorAssign<[usize; ELEMENT_COUNT]> for Bitmap1024 {
    fn bitxor_assign(&mut self, rhs: [usize; ELEMENT_COUNT]) {
        for i in 0..ELEMENT_COUNT {
            self.0[i] ^= rhs[i];
        }
    }
}

// Traits implementing arithmetic operations between Bitmaps and their respective integer types.

impl Add<usize> for Bitmap1024 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        let mut bitmap = self.0;
        let mut carry = rhs;

        for i in (0..ELEMENT_COUNT).rev() {
            if usize::MAX - carry < bitmap[i] {
                bitmap[i] = bitmap[i].wrapping_add(carry);
                carry = 1;
            } else {
                bitmap[i] += carry;
                carry = 0;
                break;
            }
        }

        if carry > 0 {
            eprintln!("Warning: Adding led to overflow!");
        }

        Self(bitmap)
    }
}

impl AddAssign<usize> for Bitmap1024 {
    fn add_assign(&mut self, rhs: usize) {
        let mut carry = rhs;

        for i in (0..ELEMENT_COUNT).rev() {
            if usize::MAX - carry < self.0[i] {
                self.0[i] = self.0[i].wrapping_add(carry);
                carry = 1;
            } else {
                self.0[i] += carry;
                carry = 0;
                break;
            }
        }

        if carry > 0 {
            eprintln!("Warning: Adding led to overflow!");
        }
    }
}

impl Deref for Bitmap1024 {
    type Target = [usize; ELEMENT_COUNT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// An attempt at serialization so far, no idea how to implement deserialisation yet
//
// impl Serialize for Bitmap1024 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(NUM_ELEMENTS))?;
//         for e in self.0 {
//             seq.serialize_element(&e)?;
//         }
//         seq.end()
//     }
// }

#[cfg(test)]
mod tests {
    use super::BitmapSize;
    use super::{Bitmap1024, ELEMENT_COUNT, ELEMENT_SIZE};
    use std::mem;

    #[test]
    fn create_default() {
        let bitmap = Bitmap1024::default();
        assert_eq!([0; ELEMENT_COUNT], *bitmap);
    }

    #[test]
    fn constants_correct() {
        assert_eq!(ELEMENT_SIZE, mem::size_of::<usize>() * 8);
        assert_eq!(Bitmap1024::MAP_LENGTH, 1_024);
        assert_eq!(ELEMENT_COUNT, Bitmap1024::MAP_LENGTH / ELEMENT_SIZE);
    }
}
