use fixed_bitmaps::Bitmap64;

fn main() {
    // Creates an empty bitmap
    let mut bitmap = Bitmap64::default();

    // Bitmaps implement Display so you can view what the map looks like
    println!("Default bitmap: {}", bitmap);

    // Bitmaps also convert to their respective unsigned int versions and back again easily
    println!("Value of bitmap: {}", bitmap.to_u64());

    // Let's do the same as above, but actually setting the values in the bitmap to something
    bitmap |= Bitmap64::from(101);

    // Will show 0000000000000000000000000000000000000000000000000000000001100101
    println!("Bitmap after OR-ing with 101: {}", bitmap);

    // Set the 4th index (the 5th bit) to true. Can simply unwrap the result to remove the warning, as we know
    // for certain that 4 < 63
    bitmap.set(4, true).unwrap();

    // Will show that 117 (101 + 2^4) is the value of the bitmap
    println!("Bitmap value: {}", bitmap.to_u64());

    // Will print out the error thrown when trying to set an index out of bounds
    match bitmap.set(64, true) {
        Ok(_) => println!("That wasn't meant to happen... something's up with my implementation!"),
        Err(error) => {
            println!("Yep, threw an error as expected. Error message is as follows:");
            eprintln!("{}", error);
        }
    }

    // Multiple ways to create a new bitmap
    let _empty = Bitmap64::default();
    let _full = Bitmap64::from(u64::MAX);

    // Equivalent ways to create a bitmap with last bits 1001
    let _bitmap = Bitmap64::from(9);
    let _bitmap = Bitmap64::from(0b1001);

    // Sets the 7th least significant bit when creating a new bitmap (indexing starts at 0)
    let mut bitmap = Bitmap64::from_set(6).unwrap();

    // Use the set() method to work with specific bits
    bitmap.set(6, false).unwrap();
    bitmap.set(42, true).unwrap();

    // Use get() to know the value of a specific bit
    println!("Bit at index 42: {}", bitmap.get(42).unwrap());

    // Freely use boolean operators &, |, and ^
    let bitmap1 = Bitmap64::from(0b1001);
    let bitmap2 = Bitmap64::from(0b1010);

    let _and = bitmap1 & bitmap2;
    let _or = bitmap1 | bitmap2;
    let _xor = bitmap1 ^ bitmap2;

    // Aritmetic operators are currently used as exactly that, the following is guarunteed to continue working as it does
    let _add = bitmap1 + 10;
    let _sub = bitmap1 - 4;
    let _mul = bitmap2 * 2;
    let _div = bitmap2 / 2;

    // The following works exactly as above, but is likely to change in favour of set operations in the major update to 1.0.0
    let _add = bitmap1 + Bitmap64::from(10);
    let _sub = bitmap1 - Bitmap64::from(4);
    let _mul = bitmap2 * Bitmap64::from(2);
    let _div = bitmap2 / Bitmap64::from(2);
}
