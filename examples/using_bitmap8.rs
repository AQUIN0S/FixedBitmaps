use fixed_bitmaps::{Bitmap128, Bitmap8, BitmapSize};

fn main() {
    // Creates an empty bitmap
    let mut bitmap = Bitmap8::default();

    // Bitmaps implement Display so you can view what the map looks like
    println!("Default bitmap: {}", bitmap);

    // Bitmaps also convert to their respective unsigned int versions and back again easily
    println!("Value of bitmap: {}", bitmap.to_u8());

    // Let's do the same as above, but actually setting the values in the bitmap to something
    bitmap |= Bitmap8::from(101);

    // Will show 01100101
    println!("Bitmap after OR-ing with 101: {}", bitmap);

    // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to remove the warning, as we know
    // for certain that 4 < 63
    bitmap.set(4, true).unwrap();

    // Will show that 117 (101 + 2^4) is the value of the bitmap
    println!("Bitmap value: {}", bitmap.to_u8());

    // Will print out the error thrown when trying to set an index out of bounds
    match bitmap.set(8, true) {
        Ok(_) => println!("That wasn't meant to happen... something's up with my implementation!"),
        Err(error) => {
            println!("Yep, threw an error as expected. Error message is as follows:");
            eprintln!("{}", error);
        }
    }

    let a = Bitmap8::from_set(2).unwrap();

    // The above is equivalent to:
    let b = Bitmap8::from(0b100);

    assert!(a == b);

    let bitmap = Bitmap8::create_bit_mask(3, 6, true);
    println!("{}", bitmap);
    println!("{}", *bitmap);
    println!("{}", 0b111000);

    println!("{:b}", u8::MAX << 3);

    let bitmap = Bitmap8::create_bit_mask(3, 6, false);
    println!("{}", bitmap);
    println!("{:b}", u8::MAX.wrapping_shl(8));

    let a = Bitmap128::create_bit_mask(3, 6, false);
    let b = Bitmap128::create_bit_mask(7, 8, false);
    let c = Bitmap128::create_bit_mask(0, 1, false);
    let d = Bitmap128::create_bit_mask(0, 0, false);
    let e = Bitmap128::create_bit_mask(8, 8, false);
    let f = Bitmap128::create_bit_mask(0, Bitmap128::MAP_LENGTH, false);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
}
