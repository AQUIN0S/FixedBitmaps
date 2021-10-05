use fixed_bitmaps::Bitmap8;

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
}
