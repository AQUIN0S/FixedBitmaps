//! # Fixed Bitmaps
//!
//! This is a crate whose aim is to create the simplest bitmap structures to work with. Currently there is only the one
//! bitmap struct to work with, `Bitmap64`, though there are plans to add more in the future. See `Bitmap64` documentation to see
//! how to use it.
//!
//! Note that indexing for bit access starts at 0, which allows you to know what the effect of setting a bit will be, by putting 2 to
//! the power of the index. For example, the following example sets the 5th bit to true in an otherwise empty bitmap. This is equivalent
//! to adding 2<sup>5</sup> () to the underlying value:
//! ```rust
//! using fixed_bitmaps::Bitmap64;
//!
//! let mut bitmap = Bitmap64::default;
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

mod bitmap64;
mod bitmap8;

pub use bitmap64::Bitmap64;
pub use bitmap8::Bitmap8;
