//! # Fixed Bitmaps
//!
//! This is a crate whose aim is to create the simplest bitmap structures to work with. This crate provides wrappings for Rust unsigned
//! integers from u8 up to u128, along with usize.
//!
//! Note that indexing for bit access starts at 0, which allows you to know what the effect of setting a bit will be, by putting 2 to
//! the power of the index. For example, the following example sets the 5th bit to true in an otherwise empty bitmap. This is equivalent
//! to adding 2<sup>5</sup> () to the underlying value:
//! ```rust
//! use fixed_bitmaps::Bitmap64;
//!
//! let mut bitmap = Bitmap64::default();
//!
//! // Set the 5th index (the 6th bit) to true.
//! // Can simply unwrap the result to remove the warning, as we know
//! // for certain that 5 < 64
//! bitmap.set(5, true).unwrap();
//!
//! // The following will throw an error however, as the 64th index is out of bounds
//! // (the highest index in a `Bitmap64` is 63)
//!
//! // Will print out the error thrown when trying to set an index out of bounds
//! match bitmap.set(64, true) {
//!     Ok(_) => println!("That wasn't meant to happen... something's up with my implementation!"),
//!     Err(error) => {
//!         println!("Yep, threw an error as expected. Error message is as follows:");
//!         eprintln!("{}", error);
//!     }
//! }
//! ```
//!
//! # More Examples
//!
//! ```rust
//! use fixed_bitmaps::Bitmap64;
//!
//! // Multiple ways to create a new bitmap
//! let _empty = Bitmap64::default();
//! let _full = Bitmap64::from(u64::MAX);
//!
//! // Equivalent ways to create a bitmap with last bits 1001
//! let bitmap = Bitmap64::from(9);
//! let bitmap = Bitmap64::from(0b1001);
//!
//! // Sets the 7th least significant bit when creating a new
//! // bitmap (indexing starts at 0)
//! let mut bitmap = Bitmap64::from_set(6).unwrap();
//!
//! // Use the set() method to work with specific bits
//! bitmap.set(6, false).unwrap();
//! bitmap.set(42, true).unwrap();
//!
//! // Use get() to know the value of a specific bit
//! println!("Bit at index 42: {}", bitmap.get(42).unwrap());
//!
//! // Freely use boolean operators &, |, and ^
//! let bitmap1 = Bitmap64::from(0b1001);
//! let bitmap2 = Bitmap64::from(0b1010);
//!
//! let _and = bitmap1 & bitmap2;
//! let _or = bitmap1 | bitmap2;
//! let _xor = bitmap1 ^ bitmap2;
//!
//! // The following also works exactly the same
//! let _and = bitmap1 & 0b1010;
//! let _or = bitmap1 | 0b1010;
//! let _xor = bitmap1 ^ 0b1010;
//!
//! // Aritmetic operators are currently used as exactly that, the following is
//! // guarunteed to continue working as it does
//! let _add = bitmap1 + 10;
//! let _sub = bitmap1 - 4;
//! let _mul = bitmap2 * 2;
//! let _div = bitmap2 / 2;
//!
//! // The following work exactly as above, but are likely to change in favour of
//! // set operations in the major update to 1.0.0
//! let _add = bitmap1 + Bitmap64::from(10);
//! let _sub = bitmap1 - Bitmap64::from(4);
//! let _mul = bitmap2 * Bitmap64::from(2);
//! let _div = bitmap2 / Bitmap64::from(2);
//!
//! // Left and right shifts work exactly as they do with integers
//! let _lsh = bitmap1 << 3;
//! let _rsh = bitmap2 >> 1;
//! ```

mod bitmap128;
mod bitmap16;
mod bitmap32;
mod bitmap64;
mod bitmap8;
mod bitmap_arch;

pub use bitmap128::Bitmap128;
pub use bitmap16::Bitmap16;
pub use bitmap32::Bitmap32;
pub use bitmap64::Bitmap64;
pub use bitmap8::Bitmap8;
pub use bitmap_arch::BitmapArch;
