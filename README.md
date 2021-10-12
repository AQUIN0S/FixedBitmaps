# Fixed Bitmaps

[![Build Status](https://app.travis-ci.com/AQUIN0S/FixedBitmaps.svg?branch=main)](https://github.com/AQUIN0S/FixedBitmaps)
[![API](https://docs.rs/fixed_bitmaps/badge.svg)](https://docs.rs/fixed_bitmaps)
[![crates.io](https://img.shields.io/crates/v/fixed_bitmaps)](https://crates.io/crates/fixed_bitmaps)
[![Downloads](https://img.shields.io/crates/d/fixed_bitmaps)](https://crates.io/crates/fixed_bitmaps)
[![License](https://img.shields.io/crates/l/fixed_bitmaps)](https://opensource.org/licenses)

Implement bitmap functionality around primitive Rust unsigned integers. I ended up making this after wanting a simple data structure to use for bit flags. Other options definitely exist, but some looked intimidating and others I didn't find till after I made this, and besides, I just wanted to make a crate I was willing to publish!

These bitmaps are simply for when you want a data structure to hold boolean flags, which can be AND-ed, OR-ed and XOR-ed together, in as compressed a format as possible, while still holding enough functionality to easily view the bitmap for display, or get a particular bit. Working with this crate is a breeze, and if you want a relatively small dependency that's stable, lightning fast, and easy to use, this could be the one for you!

One thing this crate is not is *flexible*! The largest bitmaps you can make with this are 128 bits long, so if you want something larger, you may wish to visit other crates that implement this. The one that I would recommend, which this crate turned out to become very similar to, is [bit-vec](https://crates.io/crates/bit-vec).

## Features

- There are now wrappings for all basic types of unsigned integers, from `u8` up to `u128`, along with a wrapper for `usize` (`BitmapArch`).
- Both bitwise (and, or, xor, not) and arithmetic (add, subtract, multiply, divide) operations supported between bitmaps, and between bitmaps and their respective integer type.
- Implements `Display` to show the bitmap in all its 1's and 0's glory. (May end up changing this to the `Debug` trait however, that'll have to be something to think about before releasing 1.0.0).
- Easy conversion between a `Bitmap` and the integer type it's associated with using a dedicated method or the dereference operator `*`.
- Left and right shifts now implemented

## Code examples

```rust
use fixed_bitmaps::Bitmap64;

// Multiple ways to create a new bitmap
let empty = Bitmap64::default();
let full = Bitmap64::from(u64::MAX);

// Equivalent ways to create a bitmap with last bits 1001
let bitmap = Bitmap64::from(9);
let bitmap = Bitmap64::from(0b1001);

// Sets the 7th least significant bit when creating a new
// bitmap (indexing starts at 0)
let mut bitmap = Bitmap64::from_set(6).unwrap();

// Use the set() method to work with specific bits
bitmap.set(6, false).unwrap();
bitmap.set(42, true).unwrap();

// Use get() to know the value of a specific bit
println!("Bit at index 42: {}", bitmap.get(42).unwrap());

// Freely use boolean operators &, |, and ^
let bitmap1 = Bitmap64::from(0b1001);
let bitmap2 = Bitmap64::from(0b1010);

let and = bitmap1 & bitmap2;
let or = bitmap1 | bitmap2;
let xor = bitmap1 ^ bitmap2;

// The following also works exactly the same
let and = bitmap1 & 0b1010;
let or = bitmap1 | 0b1010;
let xor = bitmap1 ^ 0b1010;

// You can also use the not operator ! to get the flipped version of the bitmap
let flipped = !bitmap;

// Aritmetic operators are currently used as exactly that, the following
// is guarunteed to continue working as it does
let add = bitmap1 + 10;
let sub = bitmap1 - 4;
let mul = bitmap2 * 2;
let div = bitmap2 / 2;

// The following works exactly as above, but is likely to change in
// favour of set operations in the major update to 1.0.0
let add = bitmap1 + Bitmap64::from(10);
let sub = bitmap1 - Bitmap64::from(4);
let mul = bitmap2 * Bitmap64::from(2);
let div = bitmap2 / Bitmap64::from(2);

// Left and right shifts work exactly as they do with integers
let lsh = bitmap1 << 3;
let rsh = bitmap2 >> 1;
```

## To be done

- Documentation is always something to be improved, any documentation changes or minor bug fixes result in a patch update.
- Anytime a new structure is defined and implemented, or a new function defined, a minor update will be published.
- When this has been fully tested and benchmarked, and any other important components I might have missed have been added, I will publish fixed_bitmaps 1.0.0 as a major update, to indicate its readyness for full production.

## Contributions

Contributions are always welcome, whether for better documentation, bugfixing or optimizations in the code itself!

When making changes, only do so in the following two files (unless of course you plan to add a new module or something of the sort):

- `fixed_bitmaps/src/bitmap128.rs`
- `fixed_bitmaps/tests/test_bitmap128.rs`

You can then run `cargo run` on this project, and it will propogate changes to those files through to all of the other files in their
respective directories, adapting the changes to match the particular primitive each module aims to support.
