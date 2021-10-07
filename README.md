# Fixed Bitmaps

A small crate implementing bitmap functionality around primitive Rust unsigned integers. I ended up making this after wanting a simple data structure to use for bit flags. Other options definitely exist such as `bitmap` and `bitmaps`, but they looked intimidating, and besides, I just wanted to make a crate I was willing to publish!

These bitmaps are simply for when you want a data structure to hold boolean flags, which can be AND-ed, OR-ed and XOR-ed together, in as compressed a format as possible, while still holding enough functionality to easily view the bitmap for display, or get a particular bit.

## Features

- There are now wrappings for all basic types of unsigned integers, from `u8` up to `u128`, along with a wrapper for `usize` (`BitmapArch`).
- Both bitwise (and, or, xor) and arithmetic (add, subtract, multiply, divide) operations supported between bitmaps, and between bitmaps and their respective integer type.
- Implements `Display` to show the bitmap in all its 1's and 0's glory. (May end up changing this to the `Debug` trait however, that'll have to be something to think about before releasing 1.0.0).
- Easy conversion between a `Bitmap` and the integer type it's associated with. For example, a `Bitmap64` and a `u64` can easily be converted back and forth like the following:

    ```rust
    use fixed_bitmaps::Bitmap64;

    let mut bitmap = Bitmap::from(53);
    
    // Use and mutate bitmap as much as you like
    
    // Get the resulting u64 value back easily
    let final_value = bitmap.to_u64();
    ```

## To be done

- Documentation is always something to be improved, any documentation changes or minor bug fixes result in a patch update.
- Anytime a new structure is defined and implemented, a minor update will be published.
- When this has been fully tested and benchmarked, and any other important components I might have missed have been added, I will publish fixed_bitmaps 1.0.0 as a major update, to indicate its readyness for full production.
- Bitwise shifts would probably be a good next step.
- I'm also considering using the dereference operator as an extra option to convert a bitmap back to its integer type. This will be one thing to consider before moving to 1.0.0

## Contributions

Contributions are always welcome, whether for better documentation, bugfixing or optimizations in the code itself!