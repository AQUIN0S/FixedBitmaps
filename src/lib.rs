//! # Fixed Bitmaps
//!
//! This is a crate whose aim is to create the simplest bitmap structures to work with. Currently there is only the one
//! bitmap struct to work with, `Bitmap64`, though there are plans to add more in the future.
//!
//! # Examples
//! ```rust
//! // Creates an empty bitmap
//! let mut bitmap = Bitmap64::default();
//!
//! // Bitmaps implement Display so you can view what the map looks like
//! // Will show 0000000000000000000000000000000000000000000000000000000000000000
//! println!("Default bitmap: {}", bitmap);
//!
//! // Bitmaps also convert to their respective unsigned int versions and back again easily
//! // Will show 0 as the value of the bitmap
//! println!("Value of bitmap: {}", bitmap.to_u64());
//!
//! // Let's do the same as above, but actually setting the values in the bitmap to something
//! bitmap |= Bitmap64::from(101);
//!
//! // Will show 0000000000000000000000000000000000000000000000000000000001100101
//! println!("Bitmap after OR-ing with 101: {}", bitmap);
//!
//! // Will show that 101 is the value of the bitmap
//! println!("Bitmap value: {}", bitmap.to_u64());
//! ```

mod bitmap64;

pub use bitmap64::Bitmap64;
